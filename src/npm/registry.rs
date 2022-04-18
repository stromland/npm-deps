use std::str::FromStr;

use futures::future::join_all;
use reqwest::Url;
use serde::Deserialize;
use tokio::task::JoinHandle;

use crate::npm::client::NpmConfig;
use crate::npm::dependency::Dependency;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

pub struct NpmRegistry {
    pub config: NpmConfig,
}

impl NpmRegistry {
    pub fn new(config: NpmConfig) -> NpmRegistry {
        NpmRegistry { config }
    }

    pub async fn get_latest_version(
        &self,
        dependencies: Vec<Dependency>,
    ) -> Vec<(Dependency, Option<String>)> {
        let base_url = self.get_base_url();
        let tasks: Vec<JoinHandle<(Dependency, Option<String>)>> = dependencies
            .into_iter()
            .map(|dep| {
                let base = base_url.clone();
                tokio::spawn(async move {
                    match NpmRegistry::fetch_dist_tag(&base, &dep).await {
                        Some(dist_tags) => (dep, Some(dist_tags.latest)),
                        None => (dep, None),
                    }
                })
            })
            .collect();

        join_all(tasks)
            .await
            .into_iter()
            .flat_map(|res| match res {
                Ok(it) => Some(it),
                Err(e) => {
                    log::warn!("{}", e);
                    None
                }
            })
            .collect()
    }

    async fn fetch_dist_tag(base_url: &Url, dep: &Dependency) -> Option<DistTags> {
        match reqwest::get(NpmRegistry::get_dist_tags_url(base_url, &dep.name)).await {
            Ok(res) => match res.json::<DistTags>().await {
                Ok(dist_tags) => Some(dist_tags),
                Err(e) => {
                    log::warn!("Failed deserialize response for {} - {}", &dep.name, e);
                    None
                }
            },
            Err(e) => {
                log::warn!("Failed fetching dist-tags for {} - {}", &dep.name, e);
                None
            }
        }
    }

    fn get_dist_tags_url(base_url: &Url, name: &str) -> Url {
        let path = format!("-/package/{}/dist-tags", name);
        base_url.join(&path).unwrap()
    }

    fn get_base_url(&self) -> Url {
        Url::from_str(&self.config.registry).unwrap()
    }
}
