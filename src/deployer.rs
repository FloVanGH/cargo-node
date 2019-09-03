use std::{fs, io::Write};

use sigma::Sigma;

use crate::{
    builder::save_template, cargo_toml::*, command::Command, config::*, node_toml::*, templates::*,
};

/// Deploys the project with the given settings.
pub struct Deployer;

impl Deployer {
    /// Creates a new deployer.
    pub fn new() -> Self {
        Deployer
    }

    /// Runs the build process. Returns the output director.
    pub fn run(&self, config: &Config, cargo_toml: &CargoToml, output_dir: &str) {
        // let mut path_extension = "";
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
            }
            Package::Example(bin) => {
                app_name = bin.into();
            }
            _ => {}
        }

        let output_sub = output_dir.to_string().replace("target/", "");

        match &config.target {
            Target::Electron => {
                println!("\nelectron-packager");
                Command::new("electron-packager")
                    .current_dir("target/")
                    .arg(output_sub)
                    .arg(app_name)
                    .output()
                    .expect("Could not run electron-packager.");
            }
            Target::Browser => {
                println!("\ndeploy browser target");
                let deploy_path = format!("target/{}-browser", app_name);
                fs::create_dir_all(deploy_path.as_str()).unwrap();
                Command::new("wasm2js")
                    .arg(format!("{}/{}.wasm", output_dir, app_name))
                    .arg("-o")
                    .arg(format!("{}/{}.wasm.js", deploy_path, app_name))
                    .output()
                    .expect("Could not run electron-packager.");

                let index_html = Sigma::new(BROWSER_INDEX_HTML_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .parse()
                    .expect("Could not parse index.html template.")
                    .compile()
                    .expect("Could not compile index.hml template.");

                save_template(index_html, format!("{}/index.html", deploy_path));

                let app_js = Sigma::new(CARGO_WEB_BROWSER_JS)
                    .parse()
                    .expect("Could not parse app js template.")
                    .compile()
                    .expect("Could not compile app js template.");

                save_template(app_js, format!("{}/{}.js", deploy_path, app_name));
            }
            Target::Android => {
                // todo
            }
            _ => {}
        }
    }
}
