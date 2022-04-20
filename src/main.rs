use log::LevelFilter;
use simple_logger::SimpleLogger;

use npm_deps::npm::client::Npm;
use npm_deps::{Dependency, SemverUpgrade};

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .env()
        .init()
        .unwrap();

    let outdated = Npm::outdated().expect("could not run npm outdated");

    let mut dependencies: Vec<Dependency> = outdated
        .into_iter()
        .map(|(key, value)| {
            let current = semver::Version::parse(&value.wanted).unwrap();
            let latest = semver::Version::parse(&value.latest).unwrap();

            let semver = SemverUpgrade::evaluate(&current, &latest);

            Dependency {
                name: key,
                current_version: value.current.clone(),
                wanted_version: value.wanted.clone(),
                latest_version: value.latest.clone(),
                is_dev: value.is_dev(),
                upgrade: semver,
            }
        })
        .collect();

    // 1. Sort by PATCH, MINOR, MAJOR
    // 2. Sort by non-dev and dev dependencies
    // 3. Sort by name
    dependencies.sort_by(|d1, d2| {
        d2.upgrade
            .cmp(&d1.upgrade)
            .then(d1.is_dev.cmp(&d2.is_dev))
            .then(d1.name.cmp(&d2.name))
    });

    let current_is_empty = dependencies.iter().all(|dep| dep.current_version.is_none());

    if current_is_empty {
        log::warn!("you should install all dependencies first");
    }

    let total = dependencies.len();
    log::info!("{} dependencies available for update", total);

    if total == 0 {
        println!("All dependencies are up to date!")
    } else {
        let table = npm_deps::table::get_dependency_table(dependencies);
        println!("{}", table);
    }
}
