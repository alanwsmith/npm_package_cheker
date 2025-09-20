#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
struct SuspectPackages {
    packages: BTreeMap<String, SuspectPackage>,
}

impl SuspectPackages {
    pub fn new() -> SuspectPackages {
        SuspectPackages {
            packages: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, package_name: &String) {}
}

#[derive(Debug)]
struct SuspectPackage {
    suspect_versions: BTreeMap<String, SuspectVersion>,
}

impl SuspectPackage {
    pub fn new() -> SuspectPackage {
        SuspectPackage {
            suspect_versions: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
struct SuspectVersion {
    dependencies: Vec<String>,
}

impl SuspectVersion {
    pub fn new() -> SuspectVersion {
        SuspectVersion {
            dependencies: vec![],
        }
    }
}

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
        Ok(DateTime::parse_from_rfc3339(earliest_string)?)
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
    let target_package = "minify".to_string();
    let mut suspects = SuspectPackages::new();
    suspects.add(&target_package);
    Ok(())
}

fn load_back_packages() -> Result<BadPackages> {
    let mut bad_packages = BadPackages::new();
    let input = fs::read_to_string("bad-packages.json")?;
    bad_packages.packages = serde_json::from_str(&input)?;
    Ok(bad_packages)
}

fn get_json(url: &str) -> Result<Value> {
    let res = reqwest::blocking::get(url)?;
    if res.status() == 200 {
        let body = res.text()?;
        let data = serde_json::from_str::<Value>(&body)?;
        Ok(data)
    } else {
        Err(anyhow!("Could not get JSON"))
    }
}
