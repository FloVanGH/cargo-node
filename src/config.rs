use std::{env, fs::File, io::prelude::*};

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct CargoPackage {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    package: Option<CargoPackage>,
}

#[derive(Debug, PartialEq)]
pub enum CargoMode {
    Bin,
    Example(String),
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Build,
    Run,
    // Deploy
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Build
    }
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "build" => {
                Mode::Build
            },
            "run" => {
                Mode::Run
            },
            _ => {
                panic!("Unknown mode: {}", s);
            }
        }
    }
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        Mode::from(s.as_str())
    }
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Electron,
    Browser,
    Android,
    IOS,
}

impl Default for Target {
    fn default() -> Self {
        Target::Electron
    }
}

impl From<&str> for Target {
    fn from(s: &str) -> Self {
        match s {
            "electron" => {
                Target::Electron
            },
            "browser" => {
                Target::Browser
            },
            "android" => {
                Target::Android
            },
            "ios" => {
                Target::IOS
            }
            _ => {
                panic!("Unknown target: {}", s);
            }
        }
    }
}

impl From<String> for Target {
    fn from(s: String) -> Self {
        Target::from(s.as_str())
    }
}

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub target: Target,

    pub cargo_mode: CargoMode,
    pub path_extension: String,
    pub name: String,
}

impl Config {
    pub fn new() -> Self {
        let mut is_example = false;
        let mut path_extension = "";
        let mut cargo_mode = CargoMode::Bin;

        let toml_file = File::open("Cargo.toml");

        if toml_file.is_err() {
            panic!("Could not load Cargo.toml");
        }

        let mut contents = String::new();
        toml_file.unwrap().read_to_string(&mut contents).unwrap();

        let cargo_toml: CargoToml = toml::from_str(contents.as_str()).unwrap();

        let mut name = cargo_toml.package.unwrap().name.unwrap();

        // read command line arguments
        for argument in env::args() {
            match argument.as_str() {
                "--example" => {
                    is_example = true;
                }
                _ => {
                    if is_example {
                        name = argument.to_string();
                        cargo_mode = CargoMode::Example(argument);
                        path_extension = "/examples";
                        is_example = false;
                    }
                }
            };
        }
        // read project toml file

        Config {
            cargo_mode,
            path_extension: path_extension.to_string(),
            name,
            mode: Mode::Build,
            target: Target::Electron
        }
    }
}
