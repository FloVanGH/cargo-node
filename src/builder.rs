use std::{fs, io::prelude::*, path::Path};

use sigma::*;

use crate::{
    asset_builder::AssetBuilder, cargo_toml::*, command::Command, config::*, node_toml::*,
    templates::*,
};

/// Builds the project with the given settings.
pub struct Builder;

pub fn save_template(template: String, path: impl Into<String>) {
    let path = path.into();
    println!("\t{}", path);
    let mut file = fs::File::create(path.clone())
        .unwrap_or_else(|_| panic!("Could not create {} file.", path));
    file.write_all(template.as_bytes())
        .unwrap_or_else(|_| panic!("Could not write to {}", path));
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder
    }

    /// Runs the build process. Returns the output director.
    pub fn run(
        &self,
        config: &Config,
        cargo_toml: &CargoToml,
        node_toml: &Option<NodeToml>,
        asset_builder: &AssetBuilder,
    ) -> String {
        // build with cargo web
        let mut cargo_web_command = Command::new("cargo-web").arg("build");
        let mut path_extension = "";
        let mode_path = "debug";
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

        // if config.mode == Mode::Release || config.task == Task::Deploy {
        //     cargo_web_command = cargo_web_command.arg("--release");
        //     mode_path = "release";
        // }

        println!("Run cargo-web.");
        cargo_web_command
            .output()
            .expect("Could not build with cargo-web.");

        let cargo_web_output_dir = format!(
            "target/wasm32-unknown-unknown/{}{}",
            mode_path, path_extension
        );

        if config.target == Target::Browser {
            // todo: remove redundant code
            let mut style = String::new();
            if let Some(node_toml) = node_toml {
                if let Some(fonts) = node_toml.fonts(app_name.as_str()) {
                    let mut work_around = String::new();
                    style.push_str("<style>");

                    for font in fonts {
                        let path = Path::new(font.src.as_str());
                        let font_file =
                            format!("fonts/{}", path.file_name().unwrap().to_str().unwrap());

                        style.push_str(
                            format!(
                                "@font-face {{ font-family: '{}'; src: url('{}'); }} ",
                                font.font_family, font_file
                            )
                            .as_str(),
                        );

                        work_around.push_str(
                            format!("\n<link rel='preload' as='font' href='{}'>", font_file)
                                .as_str(),
                        )
                    }
                    style.push_str("</style>");
                    style.push_str(work_around.as_str());
                }
            }

            let index_html = Sigma::new(DEFAULT_INDEX_HTML_TEMPLATE)
                .bind("name", app_name.as_str())
                .bind("style", style.as_str())
                .parse()
                .expect("Could not parse index.html template.")
                .compile()
                .expect("Could not compile index.hml template.");

            if config.task != Task::Deploy {
                fs::create_dir_all("static").unwrap();
                save_template(index_html, "static/index.html");
            }

            if let Some(node_toml) = node_toml {
                asset_builder.build(app_name.as_str(), "static", &node_toml);
            }

            return cargo_web_output_dir;
        }

        // copy all cargo-web output files to cargo-node output dir
        let cargo_node_output_dir = format!("target/electron{}", path_extension);
        fs::create_dir_all(cargo_node_output_dir.clone()).unwrap();

        if let Target::Electron = config.target {
            println!("\ncopy files to electron/");
            println!("\t{}", format!("{}.d", app_name));
            fs::copy(
                format!("{}/{}.d", cargo_web_output_dir, app_name),
                format!("{}/{}.d", cargo_node_output_dir, app_name),
            )
            .unwrap();
            println!("\t{}", format!("{}.js", app_name));
            fs::copy(
                format!("{}/{}.js", cargo_web_output_dir, app_name),
                format!("{}/{}.js", cargo_node_output_dir, app_name),
            )
            .unwrap();
            println!("\t{}", format!("{}.wasm", app_name));
            fs::copy(
                format!("{}/{}.wasm", cargo_web_output_dir, app_name),
                format!("{}/{}.wasm", cargo_node_output_dir, app_name),
            )
            .unwrap();
        }

        // build templates
        println!("\ncreate templates");
        match config.target {
            Target::Electron => {
                let mut style = String::new();
                if let Some(node_toml) = node_toml {
                    if let Some(fonts) = node_toml.fonts(app_name.as_str()) {
                        let mut work_around = String::new();
                        style.push_str("<style>");

                        for font in fonts {
                            let path = Path::new(font.src.as_str());
                            let font_file =
                                format!("fonts/{}", path.file_name().unwrap().to_str().unwrap());

                            style.push_str(
                                format!(
                                    "@font-face {{ font-family: '{}'; src: url('{}'); }} ",
                                    font.font_family, font_file
                                )
                                .as_str(),
                            );

                            work_around.push_str(
                                format!("\n<link rel='preload' as='font' href='{}'>", font_file)
                                    .as_str(),
                            )
                        }

                        style.push_str("</style>");
                        style.push_str(work_around.as_str());
                    }
                }

                let index_html = Sigma::new(DEFAULT_INDEX_HTML_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .bind("style", style.as_str())
                    .parse()
                    .expect("Could not parse index.html template.")
                    .compile()
                    .expect("Could not compile index.hml template.");

                save_template(index_html, format!("{}/index.html", cargo_node_output_dir));
                let (width, height) = self.get_window_size(node_toml, app_name.as_str());
                // todo load width height from node toml
                let main_js = Sigma::new(MAIN_JS_TEMPLATE)
                    .bind("width", width.as_str())
                    .bind("height", height.as_str())
                    .parse()
                    .expect("Could not parse main.js template.")
                    .compile()
                    .expect("Could not compile main.js template.");

                save_template(main_js, format!("{}/main.js", cargo_node_output_dir));

                let package_json = Sigma::new(PACKAGE_JSON_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .parse()
                    .expect("Could not parse package.json template.")
                    .compile()
                    .expect("Could not compile package.json template.");

                save_template(
                    package_json,
                    format!("{}/package.json", cargo_node_output_dir),
                );

                let package_json = Sigma::new(PRELOAD_JS_TEMPLATE)
                    .parse()
                    .expect("Could not parse preload.js template.")
                    .compile()
                    .expect("Could not compile preload.js template.");

                save_template(
                    package_json,
                    format!("{}/preload.js", cargo_node_output_dir),
                );
            }
            Target::Android => {
                let cordova_output_dir = format!("target/cordova{}", path_extension);
                fs::create_dir_all(format!("{}/www", cordova_output_dir)).unwrap();

                // run wasm2js
                println!("\twasm2js");
                Command::new("wasm2js")
                    .arg(format!("{}/{}.wasm", cargo_web_output_dir, app_name))
                    .arg("-o")
                    .arg(format!("{}/www/{}.wasm.js", cordova_output_dir, app_name))
                    .output()
                    .expect("Could not run wasm2js.");

                let package_json = Sigma::new(CORDOVA_PACKAGE_JSON_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .parse()
                    .expect("Could not parse package.json template.")
                    .compile()
                    .expect("Could not compile package.json template.");

                save_template(package_json, format!("{}/package.json", cordova_output_dir));

                let config_xml = Sigma::new(CORDOVA_CONFIG_XML_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .parse()
                    .expect("Could not parse config.xml template.")
                    .compile()
                    .expect("Could not compile config.xml template.");

                save_template(config_xml, format!("{}/config.xml", cordova_output_dir));

                let compile_wasm_js = Sigma::new(CORDOVA_COMPILE_WASM_JS_TEMPLATE)
                    .parse()
                    .expect("Could not parse compile_wasm.js template.")
                    .compile()
                    .expect("Could not compile compile_wasm.js template.");

                save_template(
                    compile_wasm_js,
                    format!("{}/www/compile_wasm.js", cordova_output_dir),
                );

                let mut style = String::new();
                if let Some(node_toml) = node_toml {
                    if let Some(fonts) = node_toml.fonts(app_name.as_str()) {
                        let mut work_around = String::new();
                        style.push_str("<style>");

                        for font in fonts {
                            let path = Path::new(font.src.as_str());
                            let font_file =
                                format!("fonts/{}", path.file_name().unwrap().to_str().unwrap());

                            style.push_str(
                                format!(
                                    "@font-face {{ font-family: '{}'; src: url('{}'); }} ",
                                    font.font_family, font_file
                                )
                                .as_str(),
                            );

                            work_around.push_str(
                                format!("\n<link rel='preload' as='font' href='{}'>", font_file)
                                    .as_str(),
                            )
                        }

                        style.push_str("</style>");
                        style.push_str(work_around.as_str());
                    }
                }

                let index_html = Sigma::new(BROWSER_INDEX_HTML_TEMPLATE)
                    .bind("name", app_name.as_str())
                    .bind("style", style.as_str())
                    .parse()
                    .expect("Could not parse index.html template.")
                    .compile()
                    .expect("Could not compile index.hml template.");

                save_template(index_html, format!("{}/www/index.html", cordova_output_dir));

                let mut file =
                    fs::File::open(format!("{}/{}.js", cargo_web_output_dir, app_name)).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                let std_web_start = contents.find("var Module = {};").unwrap();
                let std_web_end = contents.find("return Module.exports;").unwrap()
                    + "return Module.exports;".len();

                let std_web_part = contents.get(std_web_start..std_web_end);

                let app_js = Sigma::new(CORDOVA_ANDROID_JS)
                    .bind("name", app_name.as_str())
                    .bind("std_web", std_web_part.unwrap())
                    .parse()
                    .expect("Could not parse app js template.")
                    .compile()
                    .expect("Could not compile app js template.");

                save_template(
                    app_js,
                    format!("{}/www/{}.js", cordova_output_dir, app_name),
                );

                // todo: fix redundant code (asset / font loading)
                if let Some(node_toml) = node_toml {
                    asset_builder.build(
                        app_name.as_str(),
                        format!("{}/www", cordova_output_dir).as_str(),
                        &node_toml,
                    );
                }

                println!("\ncordova platform add android");
                Command::new("cordova")
                    .current_dir(cordova_output_dir.clone())
                    .arg("platform")
                    .arg("add")
                    .arg("android")
                    .output()
                    .expect("Could not run cordova.");

                println!("\ncordova build android");

                let mut cordova_command = Command::new("cordova")
                    .current_dir(cordova_output_dir.clone())
                    .arg("build")
                    .arg("android");

                // if config.mode == Mode::Release || config.task == Task::Deploy {
                //     cordova_command = cordova_command.arg("--release");
                // }

                cordova_command.output().expect("Could not run cordova.");

                // run cordova
                return cordova_output_dir;
            }
            _ => {}
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

        if let Some(node_toml) = node_toml {
            asset_builder.build(
                app_name.as_str(),
                cargo_node_output_dir.to_string().as_str(),
                &node_toml,
            );
        }

        println!("\nfinished build.");

        cargo_node_output_dir
    }

    fn get_window_size(&self, node_toml: &Option<NodeToml>, app_name: &str) -> (String, String) {
        if let Some(node_toml) = node_toml {
            if let Some(windows) = &node_toml.apps {
                if windows.len() == 1 {
                    return (windows[0].width.to_string(), windows[0].height.to_string());
                } else {
                    let window = windows.iter().find(|w| w.name == app_name);

                    if let Some(window) = window {
                        return (window.width.to_string(), window.height.to_string());
                    }
                }
            }
        }

        ("100".to_string(), "100".to_string())
    }
}
