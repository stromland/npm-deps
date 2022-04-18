use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::npm::dependency::Dependency;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    #[serde(default = "HashMap::new")]
    pub dependencies: HashMap<String, String>,
    #[serde(default = "HashMap::new")]
    pub dev_dependencies: HashMap<String, String>,
}

impl PackageJson {
    pub fn read_package_json() -> Result<PackageJson, Box<dyn Error>> {
        let package_json = fs::read_to_string("package.json")?;

        let package_json: PackageJson = serde_json::from_str(package_json.as_str())?;

        Ok(package_json)
    }

    pub fn get_all_dependencies(self) -> Vec<Dependency> {
        let mut deps: Vec<Dependency> = self
            .dependencies
            .into_iter()
            .map(|(name, version)| Dependency::new(name, version, false))
            .collect();
        let dev_deps: Vec<Dependency> = self
            .dev_dependencies
            .into_iter()
            .map(|(name, version)| Dependency::new(name, version, true))
            .collect();

        deps.extend(dev_deps);

        deps
    }
}
