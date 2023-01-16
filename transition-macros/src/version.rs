use darling::FromMeta;
use syn::{Lit, NestedMeta};

#[derive(Debug)]
pub struct Versions(pub Vec<Version>);

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.major != other.major {
            return self.major.partial_cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.partial_cmp(&other.minor);
        }
        self.patch.partial_cmp(&other.patch)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromMeta for Version {
    fn from_string(value: &str) -> darling::Result<Self> {
        let mut parts = value.split('.');
        let major = parts.next().unwrap().parse().unwrap();
        let minor = parts.next().unwrap().parse().unwrap();
        let patch = parts.next().unwrap().parse().unwrap();
        Ok(Version { major, minor, patch })
    }
}

impl FromMeta for Versions {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut versions = Vec::new();
        for item in items {
            match item {
                NestedMeta::Lit(Lit::Str(str)) => {
                    versions.push(Version::from_string(&str.value())?);
                }
                _ => {}
            }
        }
        Ok(Versions(versions))
    }
}