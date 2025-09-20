#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Serialize)]
struct SuspectPackages {
    targets: Vec<String>,
    packages: BTreeMap<String, SuspectPackage>,
}

impl SuspectPackages {
    pub fn new(targets: Vec<String>) -> Result<SuspectPackages> {
        let mut sp = SuspectPackages {
            targets: targets.clone(),
            packages: BTreeMap::new(),
        };
        for target in targets.iter() {
            sp.add_suspect_package(target)?;
        }
        Ok(sp)
    }

    pub fn add_suspect_package(&mut self, package_name: &String) -> Result<()> {
        if !self.packages.contains_key(package_name) {
            println!("Getting: {}", &package_name);
            self.packages
                .insert(package_name.to_string(), SuspectPackage::new(package_name)?);
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

#[derive(Debug, Serialize)]
struct SuspectPackage {
    suspect_versions: BTreeMap<String, SuspectVersion>,
}

impl SuspectPackage {
    pub fn new(package_name: &String) -> Result<SuspectPackage> {
        let mut suspect_versions = BTreeMap::new();
        let url = format!("https://registry.npmjs.com/{}/", package_name);
        let package_details = get_json(&url)?;
        for (version_key, version_object) in package_details
            .get("versions")
            .unwrap()
            .as_object()
            .unwrap()
        {
            suspect_versions.insert(version_key.to_string(), SuspectVersion::new(version_object));
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

#[derive(Debug, Serialize)]
struct SuspectVersion {
    dependencies: BTreeMap<String, String>,
    dev_dependencies: BTreeMap<String, String>,
}

impl SuspectVersion {
    pub fn new(version_object: &Value) -> SuspectVersion {
        let mut dependencies = BTreeMap::new();
        let mut dev_dependencies = BTreeMap::new();
        if let Some(deps) = &version_object.get("dependencies") {
            for (dep_key, dep_value) in deps.as_object().unwrap() {
                dependencies.insert(dep_key.to_string(), dep_value.to_string());
                //dbg!(dep_key);
            }
        }
        if let Some(dev_deps) = &version_object.get("devDependencies") {
            for (dev_dep_key, dev_dep_value) in dev_deps.as_object().unwrap() {
                dev_dependencies.insert(dev_dep_key.to_string(), dev_dep_value.to_string());
                //dbg!(dep_key);
            }
        }
        SuspectVersion {
            dependencies,
            dev_dependencies,
        }
    }
}

// #[derive(Debug)]
// struct SuspectDependency {
//     version_number: String,
// }

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
    // TODO: Deprecated - it's not needed since just
    // pulling all versions and looking that way.
    // pub fn earliest_problem(&self) -> Result<DateTime<FixedOffset>> {
    //     let earliest_string = self
    //         .packages
    //         .iter()
    //         .max_by_key(|bad_package| {
    //             // This will panic if no date is available,
    //             // but that's fine since it means the data isn't complete
    //             DateTime::parse_from_rfc3339(
    //                 bad_package.1.versions[0].date.as_ref().unwrap().as_str(),
    //             )
    //             .unwrap()
    //         })
    //         .ok_or(anyhow!("Could not get earliers"))?
    //         .1
    //         .versions[0]
    //         .date
    //         .as_ref()
    //         .unwrap();
    //     Ok(DateTime::parse_from_rfc3339(earliest_string)?)
    // }
    //
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
    let targets = vec!["minify".to_string()];
    // let bad_packages = load_back_packages()?;
    //dbg!(bad_packages.earliest_problem());
    //let target_package = "minify".to_string();
    let suspects = SuspectPackages::new(targets)?;
    //suspects.add_suspect_package(&target_package);
    let output = serde_json::to_string_pretty(&suspects)?;
    fs::write("package-data.json", output)?;
    // dbg!(&suspects);
    Ok(())
}

// TODO: Move to the other process that does
// the check
//fn load_back_packages() -> Result<BadPackages> {
//    let mut bad_packages = BadPackages::new();
//    //let input = fs::read_to_string("bad-packages.json")?;
//    let input = include_str!("../data/bad-packages.json");
//    bad_packages.packages = serde_json::from_str(&input)?;
//    Ok(bad_packages)
//}

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
