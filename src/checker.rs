use std::{env, fs};

use crate::{command::Command, config::*};

/// Checks the prerequisites and install it if possible.
pub struct Checker;

impl Checker {
    /// Creates a new checker.
    pub fn new() -> Self {
        Checker
    }

    /// Runs the check process.
    pub fn run(&self, config: &Config) {
        // check and install if needed cargo-web
        if !self.is_program_in_path("cargo-web") {
            let output = Command::new("cargo")
                .arg("install")
                .arg("--color")
                .arg("always")
                .arg("cargo-web")
                .output()
                .expect("Could not install cargo-web.");

            println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
        }

        match config.target {
            Target::Electron => {
                self.check_npm();
            }
            // check and install if needed cordova
            Target::Android | Target::IOS => {
                if !self.is_program_in_path("cordova") {
                    self.check_npm();
                    let output = Command::new("npm")
                        .arg("install")
                        .arg("cordova")
                        .output()
                        .expect("Could not install cordova.");

                    println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
                }
            }
            _ => {}
        }
    }

    fn is_program_in_path(&self, program: &str) -> bool {
        if let Ok(path) = env::var("PATH") {
            let splitter = if cfg!(target_os = "windows") {
                ";"
            } else {
                ":"
            };

            for p in path.split(splitter) {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
        false
    }

    fn check_npm(&self) {
        if !self.is_program_in_path("npm") {
            panic!("Could not found npm in PATH.");
        }
    }
}
