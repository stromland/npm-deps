use package_json::Dependency;

pub mod package_json;
pub mod table;
pub mod npm;

pub async fn get_dependencies_to_update(dependencies: Vec<Dependency>) -> Vec<Dependency> {
    npm::registry::get_latest_version(dependencies).await.into_iter()
        .filter(|dep| dep.is_some())
        .map(|dep| dep.unwrap())
        .collect()
}
