use std::collections::HashMap;

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
        Dependency {
            name: name.clone(),
            version: version.clone(),
        }
    }
}

pub struct Dependency {
    pub name: String,
    pub version: String,
}

impl Dependency {
    pub fn new(name: String, version: String) -> Dependency {
        Dependency { name, version }
    }
    pub fn get_dist_tags_url(&self) -> String {
        format!("https://registry.npmjs.org/-/package/{}/dist-tags", self.name)
    }
}


