use std::process;

use log::LevelFilter;
use simple_logger::SimpleLogger;

use npm_deps::npm::registry::NpmRegistry;
use npm_deps::npm::{client::NpmClient, dependency::Dependency, package_json::PackageJson};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .env()
        .init()
        .unwrap();

    let package_json = PackageJson::read_package_json().unwrap_or_else(|err| {
        log::error!("expected package.json in current folder. Error: {}", err);
        process::exit(1);
    });

    let config = NpmClient::get_config().unwrap_or_else(|| {
        log::error!("could not get npm config.");
        process::exit(1);
    });

    log::info!("current npm registry: {}", &config.registry);

    let dependencies: Vec<Dependency> = package_json.get_all_dependencies();
    let total_dependencies = dependencies.len();
    log::info!("dependencies: {}", total_dependencies);

    let npm_registry = NpmRegistry::new(config);
    let mut dependencies = npm_registry
        .get_latest_version(dependencies)
        .await
        .into_iter()
        .map(|(dep, latest_version)| Dependency {
            latest_version,
            ..dep
        })
        .filter(|dep| match &dep.latest_version {
            Some(latest) => &dep.version != latest,
            None => false,
        })
        .collect::<Vec<Dependency>>();

    dependencies.sort_by(|dep1, dep2| {
        dep1.is_dev
            .cmp(&dep2.is_dev)
            .then(dep1.name.cmp(&dep2.name))
    });

    let total_updates = dependencies.len();
    log::info!(
        "{}/{} dependencies available for update",
        total_updates,
        total_dependencies
    );

    if total_updates == 0 {
        println!("All dependencies are up to date!")
    } else {
        let table = npm_deps::table::get_dependency_table(dependencies);
        println!("{}", table);
    }
}
