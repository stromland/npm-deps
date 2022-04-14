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

    pub fn get_all_dependencies(&self) -> Vec<Dependency> {
        let mut all: Vec<Dependency> = Vec::new();

        let dev_deps: Vec<Dependency> = self.dev_dependencies.iter()
            .map(PackageJson::get_dependency)
            .collect();
        let deps: Vec<Dependency> = self.dependencies.iter()
            .map(PackageJson::get_dependency)
            .collect();

        all.extend(dev_deps);
        all.extend(deps);

        all
    }

    fn get_dependency(key_value: (&String, &String)) -> Dependency {
        let (name, version) = key_value;
        Dependency::new(name.clone(), version.clone())
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
        Dependency { name, version, registry: "https://registry.npmjs.org/".to_string(), latest_version: None }
    }

    pub fn get_dist_tags_url(&self) -> String {

        format!("{}-/package/{}/dist-tags", &self.registry, &self.name)
    }
}


