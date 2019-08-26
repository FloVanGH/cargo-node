use std::{
    env,
    fs::{copy, create_dir_all, File},
    io::{prelude::*, Write},
};

use serde::Deserialize;
use sigma::Sigma;
use toml;

const DEFAULT_INDEX_HTML_TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    <script>
        var Module = {};
        var __cargo_web = {};
        Object.defineProperty( Module, 'canvas', {
            get: function() {
                if( __cargo_web.canvas ) {
                    return __cargo_web.canvas;
                }
                var canvas = document.createElement( 'canvas' );
                document.querySelector( 'body' ).appendChild( canvas );
                __cargo_web.canvas = canvas;
                return canvas;
            }
        });
    </script>
</head>
<body>
    <script src="{{ name: str }}.js"></script>
</body>
</html>"#;

const MAIN_JS_TEMPLATE: &'static str = r#"const {app, BrowserWindow} = require('electron')
const path = require('path')

let mainWindow

function createWindow () {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js')
    }
  })

  mainWindow.loadFile('index.html')

  mainWindow.on('closed', function () {
    mainWindow = null
  })
}

app.on('ready', createWindow)

app.on('window-all-closed', function () {
  app.quit()
})

app.on('activate', function () {
  if (mainWindow === null) createWindow()
})"#;

const PACKAGE_JS_TEMPLATE: &'static str = r#"{
  "name": "{{ name: str }}",
  "version": "1.0.0",
  "description": "A minimal Electron application",
  "main": "main.js",
  "scripts": {
    "start": "electron ."
  },
  "repository": "https://github.com/electron/electron-quick-start",
  "keywords": [
    "Electron",
    "quick",
    "start",
    "tutorial",
    "demo"
  ],
  "author": "GitHub",
  "license": "CC0-1.0",
  "devDependencies": {
    "electron": "^6.0.3"
  }
}"#;

const PRELOAD_JS_TEMPLATE: &'static str = r#"window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  } 
  
  for (const type of ['chrome', 'node', 'electron']) {
    replaceText(`${type}-version`, process.versions[type])
  }
})"#;

use self::command::Command;

mod command;

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
    cargo_mode: CargoMode,
}

impl Config {
    pub fn new() -> Self {
        Config {
            cargo_mode: CargoMode::Bin,
        }
    }
}

fn main() {
    let mut config = Config::new();
    let mut is_example = false;
    let mut path_extension = "";

    // read command line arguments
    for argument in env::args() {
        match argument.as_str() {
            "--example" => {
                is_example = true;
            }
            _ => {
                if is_example {
                    config.cargo_mode = CargoMode::Example(argument);
                    path_extension = "/examples";
                    is_example = false;
                }
            }
        };
    }
    // read project toml file
    let toml_file = File::open("Cargo.toml");

    if toml_file.is_err() {
        println!("Could not load Cargo.toml");
        return;
    }

    let mut contents = String::new();
    toml_file.unwrap().read_to_string(&mut contents).unwrap();

    let cargo_toml: CargoToml = toml::from_str(contents.as_str()).unwrap();

    let mut bin = cargo_toml.package.unwrap().name.unwrap();

    // Install cargo web if it is not installed.
    if !Command::new("cargo").arg("web").exists() {
        println!("\ncargo install cargo-web\n");
        let output = Command::new("cargo")
            .arg("install")
            .arg("--color")
            .arg("always")
            .arg("cargo-web")
            .output()
            .expect("Could not install cargo-web");

        let output = String::from_utf8_lossy(&output.stdout).into_owned();

        println!("{}", output);
    }

    // Build with cargo web to generate web application
    println!("\ncargo web build\n");
    let mut cargo_web_command = Command::new("cargo-web").arg("build");

    if let CargoMode::Example(s) = config.cargo_mode {
        bin = s.clone();
        cargo_web_command = cargo_web_command.arg("--example").arg(s);
    }

    // .arg("--example")
    // .arg(bin)
    cargo_web_command
        .output()
        .expect("Could not build with cargo-web.");

    let input_path = format!(
        "target/wasm32-unknown-unknown/debug{}/{}",
        path_extension, bin
    );
    let output_path = format!("target/cargo-node/debug{}", path_extension);

    // create output dir
    let _ = create_dir_all(&output_path);

    println!("{}.d", input_path);

    // copy output of cargo-web
    println!("\nCopy files to cargo-node/.\n");
    let r = copy(
        format!("{}.d", input_path),
        format!("{}/{}.d", output_path, bin),
    );
    println!("{:?}", r);
    let _ = copy(
        format!("{}.js", input_path),
        format!("{}/{}.js", output_path, bin),
    );
    let _ = copy(
        format!("{}.wasm", input_path),
        format!("{}/{}.wasm", output_path, bin),
    );

    // build electron template files
    let index_html = Sigma::new(DEFAULT_INDEX_HTML_TEMPLATE)
        .bind("name", bin.as_str())
        .parse()
        .expect("Could not parse index.html template.")
        .compile()
        .expect("Could not compile index.hml template.");

    let mut file = File::create(format!("{}/index.html", output_path))
        .expect("Could not create index.html file.");
    file.write_all(index_html.as_bytes())
        .expect("Could not write to index.html");

    let main_js = Sigma::new(MAIN_JS_TEMPLATE)
        .parse()
        .expect("Could not parse main.js template.")
        .compile()
        .expect("Could not compile main.js template.");

    let mut file =
        File::create(format!("{}/main.js", output_path)).expect("Could not create main.js file.");
    file.write_all(main_js.as_bytes())
        .expect("Could not write to main.js");

    let package_js = Sigma::new(PACKAGE_JS_TEMPLATE)
        .bind("name", bin.as_str())
        .parse()
        .expect("Could not parse package.json template.")
        .compile()
        .expect("Could not compile package.json template.");

    let mut file = File::create(format!("{}/package.json", output_path))
        .expect("Could not create package.json file.");
    file.write_all(package_js.as_bytes())
        .expect("Could not write to package.json");

    let preload_js = Sigma::new(PRELOAD_JS_TEMPLATE)
        .parse()
        .expect("Could not parse preload.js template.")
        .compile()
        .expect("Could not compile preload.js template.");

    let mut file = File::create(format!("{}/preload.js", output_path))
        .expect("Could not create preload.js file.");
    file.write_all(preload_js.as_bytes())
        .expect("Could not write to preload.js");

    // npm install
    println!("\nnpm install\n");

    Command::new("npm")
        .current_dir(format!("{}/", output_path))
        .arg("install")
        .output()
        .expect("Could not run npm install.");

    // // npm start
    // println!("Execute npm start");
    // println!("-----------------\n");
    // Command::new("npm")
    //   .current_dir(format!("{}/", output_path))
    //   .arg("start")
    //   .output()
    //   .expect("Could not run npm install.");
}
