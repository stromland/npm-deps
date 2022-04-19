use std::fmt::{Display, Formatter};

pub mod npm;
pub mod table;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SemverUpgrade {
    MAJOR,
    MINOR,
    PATCH,
}

impl Display for SemverUpgrade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_semver_upgrade(current: &semver::Version, latest: &semver::Version) -> SemverUpgrade {
    if latest.major > current.major {
        SemverUpgrade::MAJOR
    } else if latest.minor > current.minor {
        SemverUpgrade::MINOR
    } else {
        SemverUpgrade::PATCH
    }
}
