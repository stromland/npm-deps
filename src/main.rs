use std::process;

use log::LevelFilter;
use simple_logger::SimpleLogger;

use npm_deps::npm::client::NpmClient;
use npm_deps::package_json::{Dependency, PackageJson};

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(LevelFilter::Warn).env().init().unwrap();

    let package_json = PackageJson::read_package_json().unwrap_or_else(|err| {
        log::error!("expected package.json in current folder. Error: {}", err);
        process::exit(1);
    });

    let config = NpmClient::get_config().unwrap_or_else(|| {
        log::error!("could not get npm config.");
        process::exit(1);
    });

    let dependencies = package_json.get_all_dependencies()
        .into_iter()
        .map(|dep| Dependency {
            registry: config.registry.clone(),
            ..dep
        })
        .collect();

    let mut dependencies = npm_deps::get_dependencies_to_update(dependencies).await;
    dependencies.sort_by(|dep1, dep2| dep1.name.cmp(&dep2.name));

    npm_deps::table::print_dependencies(dependencies);
}
