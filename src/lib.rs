use im::hashmap::HashMap;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn valid_project_dir() -> bool {
    Command::new("yarn").arg("-v")
        .status().expect("Error: Yarn needs to be available.")
        .success() &&
        Command::new("ruby").arg("-v")
        .status().expect("Error: Ruby needs to be available.")
        .success() &&
        Command::new("node").arg("-v")
        .status().expect("Error: Node needs to be available.")
        .success() &&
        Command::new("gem").arg("which").arg("rake")
        .status().expect("Error: Rake needs to be available.")
        .success() &&
        Command::new("gem").arg("which").arg("bundler")
        .status().expect("Error: Rake needs to be available.")
        .success() &&
        Command::new("webpacker-cli")
        .status().expect("Error: WebpackerCli needs to be available.")
        .success() &&
        Command::new("rake").arg("webpacker:verify_install")
        .status().expect("Error: Webpacker not verified as installed.  Did you try `webpacker-cli init` ?")
        .success()
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

        let mut hash = <im::hashmap::HashMap<String, String>>::new();
        
        for line in BufReader::new(f).lines() {
            let line = line.unwrap();
            let result: Vec<&str> = line.split('"').collect();

            if result.len() == 5 {
                let (key, value) = (result[1], result[3]);
                hash.insert((*key).to_string(), (*value).to_string());
            }
        }

        Some(Manifest { map: hash })
    } else {
        None
    }
}

pub struct Manifest {
    map: HashMap<String, String>
}

impl Manifest {
    pub fn get(&self, key: &str)  -> Option<&String> {
        self.map.get(key)
    }
}
