use sigma::Sigma;
use std::{
  fs::{copy, create_dir_all, File},
  io::Write,
  process::{Command, Stdio},
};

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
  if (process.platform !== 'darwin') app.quit()
})

app.on('activate', function () {
  if (mainWindow === null) createWindow()
})"#;

const PACKAGE_JS_TEMPLATE: &'static str = r#"{
  "name": "{{ name: str }}a",
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

const PRELUDE_JS_TEMPLATE: &'static str = r#"window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  } 
  
  for (const type of ['chrome', 'node', 'electron']) {
    replaceText(`${type}-version`, process.versions[type])
  }
})"#;

fn main() {
  let bin = "widgets";

  // Install cargo web if it is not installed.
  if Command::new("cargo")
    .arg("web")
    .output()
    .expect("Could not run cargo")
    .status
    .code()
    .expect("Error running cargo web.")
    == 101
  {
    println!("Install cargo-web");
    println!("-----------------\n");
    let output = Command::new("cargo")
      .arg("install")
      .arg("--color")
      .arg("always")
      .arg("cargo-web")
      .stderr(Stdio::inherit())
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .output()
      .expect("Could not install cargo-web");

    let output = String::from_utf8_lossy(&output.stdout).into_owned();

    println!("{}", output);
  }

  // Build with cargo web to generate web application
  println!("Execute cargo-web");
  println!("-----------------\n");
  Command::new("cargo")
    .arg("web")
    .arg("build")
    .arg("--example")
    .arg(bin)
    .stderr(Stdio::inherit())
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .output()
    .expect("Could not build with cargo-web.");

  // cargo-node build
  println!("\nBuild with cargo-node");
  println!("----------------------\n");

  let input_path = format!("target/wasm32-unknown-unknown/debug/examples/{}", bin);
  let output_path = format!("target/cargo-node/debug/examples/{}", bin);

  // create output dir
  let _ = create_dir_all(&output_path);

  // copy output of cargo-web
  println!("Copy output of cargo-web to cargo-node.");
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
    .bind("name", bin)
    .parse()
    .expect("Could not parse index.html template.")
    .compile()
    .expect("Could not compile index.hml template.");

  let mut file =
    File::create(format!("{}/index.html", output_path)).expect("Could not create index.html file.");
  file
    .write_all(index_html.as_bytes())
    .expect("Could not write to index.html");

  let main_js = Sigma::new(MAIN_JS_TEMPLATE)
    .parse()
    .expect("Could not parse main.js template.")
    .compile()
    .expect("Could not compile main.js template.");

  let mut file =
    File::create(format!("{}/main.js", output_path)).expect("Could not create main.js file.");
  file
    .write_all(main_js.as_bytes())
    .expect("Could not write to main.js");

  let package_js = Sigma::new(PACKAGE_JS_TEMPLATE)
    .bind("name", bin)
    .parse()
    .expect("Could not parse package.json template.")
    .compile()
    .expect("Could not compile package.json template.");

  let mut file = File::create(format!("{}/package.json", output_path))
    .expect("Could not create package.json file.");
  file
    .write_all(package_js.as_bytes())
    .expect("Could not write to package.json");

  let prelude_js = Sigma::new(PRELUDE_JS_TEMPLATE)
    .parse()
    .expect("Could not parse prelude.js template.")
    .compile()
    .expect("Could not compile prelude.js template.");

  let mut file =
    File::create(format!("{}/prelude.js", output_path)).expect("Could not create prelude.js file.");
  file
    .write_all(prelude_js.as_bytes())
    .expect("Could not write to prelude.js");

  // npm install
  println!("Execute npm install");
  println!("-------------------\n");

  Command::new("npm")
    .current_dir(format!("{}/", output_path))
    .arg("install")
    .arg("--prefix")
    .arg(format!("./{}/", output_path))
    .arg(bin)
    .stderr(Stdio::inherit())
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .output()
    .expect("Could not run npm install.");

  // npm start
  // println!("Execute npm start");
  // println!("-----------------\n");
  // Command::new("npm")
  //   .current_dir(format!("{}/", output_path))
  //   .arg("start")
  //   .arg(bin)
  //   .stderr(Stdio::inherit())
  //   .stdin(Stdio::null())
  //   .stdout(Stdio::piped())
  //   .output()
  //   .expect("Could not run npm install.");
}
