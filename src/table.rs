use comfy_table::{Table};
use crate::Dependency;

pub fn print_dependencies(deps: Vec<Dependency>) {
    let mut table = Table::new();

    table.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);

    // Header
    table.set_header(vec!["Name", "Current version", "Latest version"]);

    // Dependencies
    for dep in deps {
        if let Some(latest) = dep.latest_version {
            if dep.version != latest {
                table.add_row(vec![dep.name, dep.version, latest]);
            }
        }
    }

    if table.row_iter().len() == 0 {
        println!("All dependencies are up to date!")
    } else {
        println!("{}", table);
    }
}