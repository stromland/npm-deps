use package_json::Dependency;

pub mod npm;
pub mod package_json;
pub mod table;

pub async fn get_dependencies_to_update(dependencies: Vec<Dependency>) -> Vec<Dependency> {
    npm::registry::get_latest_version(dependencies)
        .await
        .into_iter()
        .flatten()
        .filter(|dep| {
            if let Some(latest) = &dep.latest_version {
                latest != &dep.version
            } else {
                false
            }
        })
        .collect()
}
