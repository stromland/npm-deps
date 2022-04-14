use futures::future::join_all;
use serde::Deserialize;
use tokio::task::JoinHandle;

use crate::Dependency;
use crate::npm::client::NpmConfig;

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

    pub async fn fetch_dist_tag(dep: &Dependency) -> Option<DistTags> {
        match reqwest::get(dep.get_dist_tags_url()).await {
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
}

pub async fn get_latest_version(dependencies: Vec<Dependency>) -> Vec<Option<Dependency>> {
    let tasks: Vec<JoinHandle<Dependency>> = dependencies.into_iter()
        .map(|dep| {
            tokio::spawn(async move {
                match NpmRegistry::fetch_dist_tag(&dep).await {
                    Some(dist_tags) => Dependency {
                        latest_version: Some(dist_tags.latest),
                        ..dep
                    },
                    None => dep
                }
            })
        })
        .collect();

    join_all(tasks)
        .await
        .into_iter()
        .map(|res| match res {
            Ok(dep) => Some(dep),
            Err(e) => {
                log::warn!("{}", e);
                None
            }
        })
        .collect()
}
