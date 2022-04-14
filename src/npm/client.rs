use serde::{Deserialize};
use std::process;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmConfig {
    pub registry: String,
}

pub struct NpmClient { }

impl NpmClient {
    pub fn get_config() -> Option<NpmConfig> {
        let output = process::Command::new("npm")
            .args(["config", "ls", "--json"])
            .output()
            .expect("failed to run npm");

        serde_json::from_slice(&output.stdout)
            .expect("failed to read npm config")
    }
}