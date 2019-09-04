use std::{
    env,
    fs::{self, File},
    io::prelude::*,
};

use toml;

use self::builder::*;
use self::cargo_toml::*;
use self::checker::*;
use self::config::*;
use self::deployer::*;
use self::node_toml::*;
use self::runner::*;

mod builder;
mod cargo_toml;
mod checker;
mod command;
mod config;
mod deployer;
mod node_toml;
mod runner;
mod templates;

fn main() {
    // Build config file
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let config = Config::from(args.clone());

    // Clears the output.
    if config.task == Task::Clear {
        fs::remove_dir_all("target/cargo-node/").unwrap();
        fs::remove_dir_all("target/electron/").unwrap();
        fs::remove_dir_all("target/cordova/").unwrap();
        return;
    }

    // Check prerequisites
    Checker::new().run(&config);

    // Read Cargo.toml
    let mut input = String::new();

    File::open("Cargo.toml")
        .and_then(|mut f| f.read_to_string(&mut input))
        .unwrap();

    let cargo_toml: CargoToml = toml::from_str(input.as_str()).unwrap();

    // Read Node.toml (Node.toml is optional)
    let mut node_toml: Option<NodeToml> = None;
    if let Ok(toml_file) = &mut File::open("Node.toml") {
        let mut contents = String::new();
        toml_file.read_to_string(&mut contents).unwrap();

        node_toml = Some(toml::from_str(contents.as_str()).unwrap());
    }

    // run builder
    let output_dir = Builder::new().run(&config, &cargo_toml, &node_toml);

    match config.task {
        Task::Run => {
            Runner::new().run(&config, output_dir.as_str());
        }
        Task::Deploy => {
            Deployer::new().run(&config, &cargo_toml, output_dir.as_str());
        }
        _ => {}
    }
}
