#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

// #[derive(Debug)]
// struct PackageToCheck {
//     name: String,
//     suspect_versions: Vec<SuspectVersion>,
// }

// #[derive(Debug)]
// struct SuspectVersion {}

#[derive(Debug)]
struct BadPackages {
    packages: BTreeMap<String, BadPackage>,
}

impl BadPackages {
    pub fn new() -> BadPackages {
        BadPackages {
            packages: BTreeMap::new(),
        }
    }

    pub fn earliest_problem(&self) -> Result<DateTime<FixedOffset>> {
        let earliest_string = self
            .packages
            .iter()
            .max_by_key(|bad_package| {
                // This will panic if no date is available,
                // but that's fine since it means the data isn't complete
                DateTime::parse_from_rfc3339(
                    bad_package.1.versions[0].date.as_ref().unwrap().as_str(),
                )
                .unwrap()
            })
            .ok_or(anyhow!("Could not get earliers"))?
            .1
            .versions[0]
            .date
            .as_ref()
            .unwrap();
        Ok(DateTime::parse_from_rfc3339(&earliest_string)?)
    }
}

#[derive(Debug, Deserialize)]
struct BadPackage {
    versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
struct Version {
    number: String,
    date: Option<String>,
}

fn main() -> Result<()> {
    println!("Starting");
    let bad_packages = load_back_packages()?;
    dbg!(bad_packages.earliest_problem());

    Ok(())
}

fn load_back_packages() -> Result<BadPackages> {
    let mut bad_packages = BadPackages::new();
    let input = fs::read_to_string("bad-packages.json")?;
    bad_packages.packages = serde_json::from_str(&input)?;
    Ok(bad_packages)
}
