use prettytable::{Table, row, cell};
use crate::Dependency;

pub fn print_dependencies(deps: Vec<Dependency>) {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Header
    table.set_titles(row!["Name", "Current version", "Latest version"]);

    // Dependencies
    for dep in deps {
        if let Some(latest) = dep.latest_version {
            if dep.version != latest {
                table.add_row(row![dep.name, c -> dep.version, bc -> latest]);
            }
        }
    }

    if table.len() == 0 {
        println!("All dependencies are up to date!")
    } else {
        table.printstd();
    }
}