pub struct Dependency {
    pub name: String,
    pub version: String,
    pub is_dev: bool,
    pub latest_version: Option<String>,
}

impl Dependency {
    pub fn new(name: String, version: String, is_dev: bool) -> Self {
        Self {
            name,
            version,
            is_dev,
            latest_version: None,
        }
    }
}
