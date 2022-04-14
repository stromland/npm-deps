use comfy_table::Table;

use crate::Dependency;

pub fn get_dependency_table(dependencies: Vec<Dependency>) -> Table {
    let mut table = Table::new();

    table.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);

    // Header
    table.set_header(vec!["Type", "Name", "Current version", "Latest version"]);

    // Dependencies
    for dep in dependencies {
        if let Some(latest) = dep.latest_version {
            let dep_type = if dep.is_dev { "dev" } else { "" };
            table.add_row(vec![dep_type.to_string(), dep.name, dep.version, latest]);
        }
    }

    table
}
