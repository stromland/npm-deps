use comfy_table::Table;

use crate::npm::client::NpmOutdatedDetails;
use crate::SemverUpgrade;

pub fn get_dependency_table(
    dependencies: Vec<(String, NpmOutdatedDetails, SemverUpgrade)>,
) -> Table {
    let mut table = Table::new();

    table.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);

    // Header
    table.set_header(vec![
        "Upgrade",
        "Type",
        "Name",
        "Current version",
        "Latest version",
    ]);

    // Dependencies
    for (key, value, semver) in dependencies {
        let dep_type = if value.is_dev() { "dev" } else { "" };
        table.add_row(vec![
            semver.to_string(),
            dep_type.to_string(),
            key,
            value.wanted,
            value.latest,
        ]);
    }

    table
}
