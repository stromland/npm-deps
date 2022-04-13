use std::process;

use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).env().init().unwrap();

    let package_json = npm_deps::read_package_json().unwrap_or_else(|err| {
        log::error!("expected package.json in current folder. Error: {}", err);
        process::exit(1);
    });

    let mut deps = npm_deps::get_dependencies_to_update(&package_json).await;
    deps.sort_by(|dep1, dep2| dep1.name.cmp(&dep2.name));

    deps
        .into_iter()
        .for_each(|dep| log::info!("{} -> {}", dep.name, dep.version));
}
