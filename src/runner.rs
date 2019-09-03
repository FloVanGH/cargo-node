use crate::{command::Command, config::*};

/// Used to run the application.
pub struct Runner;

impl Runner {
    pub fn new() -> Self {
        Runner
    }

    pub fn run(&self, config: &Config, dir: &str) {
        match config.target {
            Target::Browser => {
                println!("\ncargo-web start.");
                let mut cargo_web_command = Command::new("cargo-web")
                    .arg("start")
                    .arg("--target=wasm32-unknown-unknown")
                    .arg("--auto-reload");

                match &config.package {
                    Package::Bin(bin) => {
                        cargo_web_command = cargo_web_command.arg("--bin").arg(bin.clone());
                    }
                    Package::Example(bin) => {
                        cargo_web_command = cargo_web_command.arg("--example").arg(bin.clone());
                    }
                    _ => {}
                }

                cargo_web_command
                    .output()
                    .expect("Could not start with cargo-web.");
            }
            Target::Electron => {
                println!("\nnpm start.");

                Command::new("npm")
                    .current_dir(format!("{}/", dir))
                    .arg("start")
                    .output()
                    .expect("Could not run npm start.");
            }
            Target::Android => {
                println!("\ncordova run android");
                Command::new("cordova")
                    .current_dir(format!("{}/", dir))
                    .arg("run")
                    .arg("android")
                    .output()
                    .expect("Could not run cordova."); 
            }
            _ => {}
        }
    }
}
