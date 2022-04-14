use std::collections::HashMap;
use std::error::Error;
use std::fs;

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
        self.dependencies.into_iter()
            .chain(self.dev_dependencies.into_iter())
            .map(PackageJson::get_dependency)
            .collect()
    }

    fn get_dependency(key_value: (String, String)) -> Dependency {
        let (name, version) = key_value;
        Dependency::new(name, version)
    }
}

pub struct Dependency {
    pub name: String,
    pub version: String,
    pub registry: String,
    pub latest_version: Option<String>,
}

impl Dependency {
    pub fn new(name: String, version: String) -> Dependency {
        Dependency {
            name,
            version,
            registry: String::from("https://registry.npmjs.org/"),
            latest_version: None,
        }
    }

    pub fn get_dist_tags_url(&self) -> String {
        format!("{}-/package/{}/dist-tags", &self.registry, &self.name)
    }
}
