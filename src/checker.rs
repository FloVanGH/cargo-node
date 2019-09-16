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
            println!("\ninstall cargo-web");
            let output = Command::new("cargo")
                .arg("install")
                .arg("--color")
                .arg("always")
                .arg("cargo-web")
                .arg("--vers")
                .arg("0.6.26")
                .output()
                .expect("Could not install cargo-web.");

            println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
        }

        match config.target {
            Target::Electron => {
                self.check_npm();

                if !self.is_program_in_path("electron") {
                    println!("\ninstall electron");
                    let output = Command::new("npm")
                        .arg("install")
                        .arg("-g")
                        .arg("electron@6.0.7")
                        .output()
                        .expect("Could not install electron-packager.");

                    println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
                }

                if config.task == Task::Deploy && !self.is_program_in_path("electron-packager") {
                    println!("\ninstall electron-packager");
                    let output = Command::new("npm")
                        .arg("install")
                        .arg("-g")
                        .arg("electron-packager@14.0.5")
                        .output()
                        .expect("Could not install electron-packager.");

                    println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
                }
            }
            // check and install wasm2js if needed
            Target::Browser => {
                self.install_wasm_2_js();
            }
            // check and install if needed cordova
            Target::Android => {
                if !self.is_program_in_path("cordova") {
                    println!("\ninstall cordova");
                    self.check_npm();
                    let output = Command::new("npm")
                        .arg("install")
                        .arg("-g")
                        .arg("cordova@9.0.0")
                        .output()
                        .expect("Could not install cordova.");

                    println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
                }

                self.install_wasm_2_js();
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
                if let Ok(dir) = fs::read_dir(p) {
                    for entry in dir {
                        if let Ok(entry) = entry {
                            if entry.file_name().into_string().unwrap().starts_with(program) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn install_wasm_2_js(&self) {
        if !self.is_program_in_path("wasm2js") {
            println!("\ninstall wasm2js");
            let output = Command::new("npm")
                .arg("install")
                .arg("-g")
                .arg("wasm2js@0.2.0")
                .output()
                .expect("Could not install wasm2js.");

            println!("{}", String::from_utf8_lossy(&output.stdout).into_owned());
        }
    }

    fn check_npm(&self) {
        if !self.is_program_in_path("npm") {
            panic!("Could not found npm in PATH.");
        }
    }
}
