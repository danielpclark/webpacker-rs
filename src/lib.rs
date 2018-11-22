extern crate im;
use im::hashmap::HashMap;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod asset_path;
// namespaces provide helpful panic info
pub mod dependency;

use dependency::validation::*;

pub type Manifest = HashMap<String, String>;

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
