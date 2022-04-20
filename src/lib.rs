use std::fmt::{Display, Formatter};

pub mod npm;
pub mod table;

pub struct Dependency {
    pub name: String,
    pub is_dev: bool,
    pub current_version: Option<String>,
    pub wanted_version: String,
    pub latest_version: String,
    pub upgrade: SemverUpgrade,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SemverUpgrade {
    MAJOR,
    MINOR,
    PATCH,
}

impl SemverUpgrade {
    pub fn evaluate(current: &semver::Version, latest: &semver::Version) -> SemverUpgrade {
        if latest.major > current.major {
            SemverUpgrade::MAJOR
        } else if latest.minor > current.minor {
            SemverUpgrade::MINOR
        } else {
            SemverUpgrade::PATCH
        }
    }
}

impl Display for SemverUpgrade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
