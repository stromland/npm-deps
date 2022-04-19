use log::LevelFilter;
use simple_logger::SimpleLogger;

use npm_deps::npm::client::Npm;
use npm_deps::npm::client::NpmOutdatedDetails;
use npm_deps::SemverUpgrade;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .env()
        .init()
        .unwrap();

    let outdated = Npm::outdated().expect("could not run npm outdated");

    let mut dependencies: Vec<(String, NpmOutdatedDetails, SemverUpgrade)> = outdated
        .into_iter()
        .map(|(key, value)| {
            let current = semver::Version::parse(&value.wanted).unwrap();
            let latest = semver::Version::parse(&value.latest).unwrap();

            let semver = npm_deps::get_semver_upgrade(&current, &latest);

            (key, value, semver)
        })
        .collect();

    dependencies.sort_by(|(d1_key, d1_value, d1_sem), (d2_key, d2_value, d2_sem)| {
        d2_sem
            .cmp(d1_sem)
            .then(d1_value.is_dev().cmp(&d2_value.is_dev()))
            .then(d1_key.cmp(d2_key))
    });

    let current_is_empty = dependencies
        .iter()
        .all(|(_, value, _)| value.current.is_none());

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
