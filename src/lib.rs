extern crate im;
use im::hashmap::HashMap;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod asset_path;

pub type Manifest = HashMap<String, String>;

// namespaces provide helpful panic info
pub mod dependency {
    pub mod validation {
        use std::process::Command;

        pub fn yarn() -> bool {
            Command::new("yarn").arg("-v").output()
                .expect("Error: Yarn needs to be available.").status.success()
        }

        pub fn ruby() -> bool {
            Command::new("ruby").arg("-v").output()
                .expect("Error: Ruby needs to be available.").status.success()
        }

        pub fn node() -> bool {
            Command::new("node").arg("-v").output()
                .expect("Error: Node needs to be available.").status.success()
        }

        pub fn rake() -> bool {
            Command::new("gem").arg("which").arg("rake").output()
                .expect("Error: Rake needs to be available.").status.success()
        }

        pub fn bundler() -> bool {
            Command::new("gem").arg("which").arg("bundler").output()
                .expect("Error: Bundler needs to be available.").status.success()
        }

        pub fn webpacker_cli() -> bool {
            Command::new("webpacker-cli").output()
                .expect("Error: WebpackerCli needs to be available.").status.success()
        }

        pub fn webpacker_initialization() -> bool {
            Command::new("rake").arg("webpacker:verify_install").output()
                .expect("Error: Webpacker not verified as installed.  Did you try `webpacker-cli init` ?")
                .status.success()
        }
    }
}

use dependency::validation::*;

pub fn valid_project_dir() -> bool {
    yarn() && ruby() && node() && rake() &&
        bundler() && webpacker_cli() &&
        webpacker_initialization()
}

pub fn compile() -> Result<(), String> {
    Command::new("webpacker-cli")
        .arg("compile")
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Error: {}", e))
}


pub fn manifest(file: Option<&str>) -> Option<Manifest> {
    let file = File::open(file.unwrap_or("public/packs/manifest.json"));

    if let Ok(f) = file {

        let mut manifest = Manifest::new();
        
        for line in BufReader::new(f).lines() {
            let line = line.unwrap();
            let result: Vec<&str> = line.split('"').collect();

            if result.len() == 5 {
                let (key, value) = (result[1], result[3]);
                manifest.insert((*key).to_string(), (*value).to_string());
            }
        }

        Some(manifest)
    } else {
        None
    }
}
