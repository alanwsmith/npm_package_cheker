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

    pub fn add_suspect_package(&mut self, package_name: &String) -> Result<()> {
        if !self.packages.contains_key(package_name) {
            println!("Getting: {}", &package_name);
            self.packages.insert(
                package_name.to_string(),
                SuspectPackage::new(&package_name)?,
            );
        } else {
            println!("Alrady have: {}", &package_name);
        }

        // let url = format!("https://registry.npmjs.com/{}/", package_name);
        // let overview = get_json(&url)?;
        // for (version_key, version_object) in overview.get("versions").unwrap().as_object().unwrap()
        // {
        //     println!("Version: {}", version_key);
        //     for (dep_key, deb_value) in version_object
        //         .get("dependencies")
        //         .unwrap()
        //         .as_object()
        //         .unwrap()
        //     {
        //         //     println!("{}", dep_key);
        //     }
        // }

        Ok(())
    }
}

#[derive(Debug)]
struct SuspectPackage {
    suspect_versions: BTreeMap<String, SuspectVersion>,
}

impl SuspectPackage {
    pub fn new(package_name: &String) -> Result<SuspectPackage> {
        let mut suspect_versions = BTreeMap::new();

        let url = format!("https://registry.npmjs.com/{}/", package_name);
        let overview = get_json(&url)?;
        for (version_key, version_object) in overview.get("versions").unwrap().as_object().unwrap()
        {
            suspect_versions.insert(version_key.to_string(), SuspectVersion::new());

            // println!("Version: {}", version_key);
            // for (dep_key, deb_value) in version_object
            //     .get("dependencies")
            //     .unwrap()
            //     .as_object()
            //     .unwrap()
            // {
            //     //     println!("{}", dep_key);
            // }
        }

        Ok(SuspectPackage { suspect_versions })
    }
}

#[derive(Debug)]
struct SuspectVersion {
    dependencies: BTreeMap<String, SuspectDependency>,
}

impl SuspectVersion {
    pub fn new() -> SuspectVersion {
        SuspectVersion {
            dependencies: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
struct SuspectDependency {
    version_number: String,
}

// impl SuspectDependency {
//     pub fn new() -> SuspectDependency {
//         SuspectDependency {
//         }
//     }
// }

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
    //dbg!(bad_packages.earliest_problem());
    let target_package = "minify".to_string();
    let mut suspects = SuspectPackages::new();
    suspects.add_suspect_package(&target_package);
    dbg!(&suspects);
    Ok(())
}

fn load_back_packages() -> Result<BadPackages> {
    let mut bad_packages = BadPackages::new();
    let input = fs::read_to_string("bad-packages.json")?;
    bad_packages.packages = serde_json::from_str(&input)?;
    Ok(bad_packages)
}

fn get_json(url: &str) -> Result<Value> {
    println!("Getting: {}", &url);
    let res = reqwest::blocking::get(url)?;
    if res.status() == 200 {
        let body = res.text()?;
        let data = serde_json::from_str::<Value>(&body)?;
        Ok(data)
    } else {
        Err(anyhow!("Could not get JSON"))
    }
}
