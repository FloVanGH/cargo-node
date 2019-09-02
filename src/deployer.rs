use crate::{cargo_toml::*, command::Command, config::*, node_toml::*, templates::*};
use std::{fs, io::Write};

/// Deploys the project with the given settings.
pub struct Deployer;

impl Deployer {
    /// Creates a new deployer.
    pub fn new() -> Self {
        Deployer
    }

    /// Runs the build process. Returns the output director.
    pub fn run(&self, config: &Config, cargo_toml: &CargoToml, node_toml: &Option<NodeToml>) {
        let mut path_extension = "";
        let mut app_name = if let Some(package) = &cargo_toml.package {
            if let Some(name) = &package.name {
                name.clone()
            } else {
                String::default()
            }
        } else {
            String::default()
        };

        match &config.package {
            Package::Bin(bin) => {
                app_name = bin.into();
                path_extension = "/bins";
            }
            Package::Example(bin) => {
                app_name = bin.into();
                path_extension = "/examples";
            }
            _ => {}
        }

        match &config.target {
            Target::Electron => {
                println!("\nelectron-packager");
                Command::new("electron-packager")
                    .current_dir("target/")
                    .arg(format!("cargo-node{}", path_extension))
                    .arg(app_name)
                    .output()
                    .expect("Could not run electron-packager.");
            }
            Target::Browser => {
                println!("\ndeploy browser target");
                let cargo_node_output_dir = format!("target/cargo-node{}", path_extension);
                let deploy_dir = format!("target/{}-browser", app_name);
                fs::create_dir_all(deploy_dir).unwrap();

                fs::copy(
                    format!("{}/{}.d", cargo_node_output_dir, app_name),
                    format!("{}/{}.d", cargo_node_output_dir, app_name),
                )
                .unwrap();        

                // todo: custom  {}.js load_wasm.js

                fs::copy(
                    format!("{}/index.js", cargo_node_output_dir),
                    format!("{}/index.js", cargo_node_output_dir),
                )
                .unwrap();
                // println!("Deployed {}");
            }
            Target::Android => {
                // todo
            }
            _ => {}
        }
    }
}
