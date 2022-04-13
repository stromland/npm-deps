use std::error::Error;
use std::fs;

use futures::future::join_all;
use serde::Deserialize;
use tokio::task::JoinHandle;

use package_json::{Dependency, PackageJson};

pub mod package_json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

pub fn read_package_json() -> Result<PackageJson, Box<dyn Error>> {
    let package_json = fs::read_to_string("package.json")?;

    let package_json: PackageJson = serde_json::from_str(package_json.as_str())?;

    Ok(package_json)
}

pub async fn get_dependencies_to_update(pkg: &PackageJson) -> Vec<Dependency> {
    let dependencies = pkg.get_all_dependencies();

    fetch_dist_tags(dependencies).await.into_iter()
        .filter(|dep| dep.is_some())
        .map(|dep| dep.unwrap())
        .collect()
}

async fn fetch_dist_tags(deps: Vec<Dependency>) -> Vec<Option<Dependency>> {
    let mut tasks: Vec<JoinHandle<Result<Dependency, String>>> = vec![];

    for dep in deps {
        tasks.push(tokio::spawn(async move {
            match reqwest::get(dep.get_dist_tags_url()).await {
                Ok(res) => match res.json::<DistTags>().await {
                    Ok(dist_tags) => Ok(Dependency {
                        name: dep.name,
                        version: dist_tags.latest,
                    }),
                    Err(e) => Err(format!("Failed deserialize response for {} - {}", dep.name, e))
                },
                Err(e) => Err(format!("Failed fetching dist-tags for {} - {}", dep.name, e))
            }
        }))
    }

    join_all(tasks).await.into_iter()
        .map(|res| match res {
            Ok(Ok(dep)) => Some(dep),
            Ok(Err(e)) => {
                log::error!("{}", e);
                None
            }
            Err(e) => {
                log::error!("{}", e);
                None
            }
        })
        .collect()
}