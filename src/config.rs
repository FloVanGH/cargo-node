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

#[derive(PartialEq)]
pub enum CargoMode {
    Bin,
    Example(String),
}

pub struct Config {
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
        }
    }
}
