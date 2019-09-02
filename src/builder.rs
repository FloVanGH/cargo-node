use sigma::*;

use crate::{cargo_toml::*, command::Command, config::*, node_toml::*, templates::*};
use std::{fs, io::Write};

/// Builds the project with the given settings.
pub struct Builder;

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder
    }

    fn save_template(&self, template: String, path: String) {
        println!("\t{}", path);
        let mut file = fs::File::create(path.clone())
            .expect(format!("Could not create {} file.", path).as_str());
        file.write_all(template.as_bytes())
            .expect("Could not write to index.html");
    }

    /// Runs the build process. Returns the output director.
    pub fn run(
        &self,
        config: &Config,
        cargo_toml: &CargoToml,
        node_toml: &Option<NodeToml>,
    ) -> String {
        // build with cargo web
        let mut cargo_web_command = Command::new("cargo-web").arg("build");
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
                cargo_web_command = cargo_web_command.arg("--bin").arg(bin.clone());
                app_name = bin.into();
                path_extension = "/bins";
            }
            Package::Example(bin) => {
                cargo_web_command = cargo_web_command.arg("--example").arg(bin.clone());
                app_name = bin.into();
                path_extension = "/examples";
            }
            _ => {}
        }

        println!("Run cargo-web.");
        cargo_web_command
            .output()
            .expect("Could not build with cargo-web.");

        // copy all cargo-web output files to cargo-node output dir
        let cargo_web_output_dir = format!("target/wasm32-unknown-unknown/debug{}", path_extension);
        let cargo_node_output_dir = format!("target/cargo-node{}", path_extension);
        fs::create_dir_all(cargo_node_output_dir.clone()).unwrap();

        println!("\ncopy files to cargo-node/.\n");

        fs::copy(
            format!("{}/{}.d", cargo_web_output_dir, app_name),
            format!("{}/{}.d", cargo_node_output_dir, app_name),
        )
        .unwrap();

        fs::copy(
            format!("{}/{}.js", cargo_web_output_dir, app_name),
            format!("{}/{}.js", cargo_node_output_dir, app_name),
        )
        .unwrap();
        fs::copy(
            format!("{}/{}.wasm", cargo_web_output_dir, app_name),
            format!("{}/{}.wasm", cargo_node_output_dir, app_name),
        )
        .unwrap();

        // build templates
        println!("\ncreate templates");
        let index_html = Sigma::new(DEFAULT_INDEX_HTML_TEMPLATE)
            .bind("name", app_name.as_str())
            .parse()
            .expect("Could not parse index.html template.")
            .compile()
            .expect("Could not compile index.hml template.");

        self.save_template(index_html, format!("{}/index.html", cargo_node_output_dir));

        // Electron templates
        if config.target == Target::Electron {
            // todo refactor
            let (width, height) = if let Some(node_toml) = node_toml {
                if let Some(windows) = &node_toml.windows {
                    if windows.len() == 1 {
                        (windows[0].width.to_string(), windows[0].height.to_string())
                    } else {
                        let window = windows
                            .iter()
                            .filter(|w| w.name.as_ref().unwrap() == app_name.as_str())
                            .next();

                        if let Some(window) = window {
                            (window.width.to_string(), window.height.to_string())
                        } else {
                            ("100".to_string(), "100".to_string())
                        }
                    }
                } else {
                    ("100".to_string(), "100".to_string())
                }
            } else {
                ("100".to_string(), "100".to_string())
            };

            // todo load width height from node toml
            let main_js = Sigma::new(MAIN_JS_TEMPLATE)
                .bind("width", width.as_str())
                .bind("height", height.as_str())
                .parse()
                .expect("Could not parse main.js template.")
                .compile()
                .expect("Could not compile main.js template.");

            self.save_template(main_js, format!("{}/main.js", cargo_node_output_dir));

            let package_json = Sigma::new(PACKAGE_JSON_TEMPLATE)
                .bind("name", app_name.as_str())
                .parse()
                .expect("Could not parse package.json template.")
                .compile()
                .expect("Could not compile package.json template.");

            self.save_template(
                package_json,
                format!("{}/package.json", cargo_node_output_dir),
            );

            let package_json = Sigma::new(PRELOAD_JS_TEMPLATE)
                .parse()
                .expect("Could not parse preload.js template.")
                .compile()
                .expect("Could not compile preload.js template.");

            self.save_template(
                package_json,
                format!("{}/preload.js", cargo_node_output_dir),
            );
        }

        // npm install
        if config.target == Target::Electron {
            println!("\nnpm install.");

            Command::new("npm")
                .current_dir(format!("{}/", cargo_node_output_dir))
                .arg("install")
                .output()
                .expect("Could not run npm install.");
        }

        println!("\nfinished build.");

        cargo_node_output_dir
    }
}
