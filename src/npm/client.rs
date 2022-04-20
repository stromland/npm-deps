use std::collections::BTreeMap;
use std::error::Error;
use std::process;

#[derive(Debug, serde::Deserialize)]
pub struct NpmOutdatedDetails {
    pub current: Option<String>,
    pub wanted: String,
    pub latest: String,
    #[serde(alias = "type")]
    pub dep_type: String,
    pub homepage: Option<String>,
}

impl NpmOutdatedDetails {
    pub fn is_dev(&self) -> bool {
        self.dep_type == "devDependencies"
    }
}

pub type NpmOutdated = BTreeMap<String, NpmOutdatedDetails>;

pub struct Npm;

impl Npm {
    pub fn outdated() -> Result<NpmOutdated, Box<dyn Error>> {
        let output = process::Command::new("npm")
            .args(["outdated", "--json", "--long"])
            .output()?;

        let outdated: NpmOutdated = serde_json::from_slice(&output.stdout)?;

        Ok(outdated)
    }
}
