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
