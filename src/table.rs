use comfy_table::Table;

use crate::Dependency;

pub fn get_dependency_table(dependencies: Vec<Dependency>) -> Table {
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
    for dep in dependencies {
        let dep_type = if dep.is_dev { "dev" } else { "" };
        table.add_row(vec![
            dep.upgrade.to_string(),
            dep_type.to_string(),
            dep.name,
            dep.wanted_version,
            dep.latest_version,
        ]);
    }

    table
}
