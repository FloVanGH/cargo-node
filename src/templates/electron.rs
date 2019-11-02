pub const MAIN_JS_TEMPLATE: &str = r#"const {app, BrowserWindow} = require('electron')
const path = require('path')

let mainWindow

function createWindow () {
  mainWindow = new BrowserWindow({
    width: {{ width }},
    height: {{ height }},
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

pub const PACKAGE_JSON_TEMPLATE: &str = r#"{
  "name": "{{ name: str }}",
  "version": "1.0.0",
  "description": "Build by cargo-node.",
  "main": "main.js",
  "scripts": {
    "start": "electron ."
  },
  "keywords": [
    "Electron",
    "Rust",
    "Wasm"
  ],
  "devDependencies": {
    "electron": "^6.0.3"
  }
}"#;

pub const PRELOAD_JS_TEMPLATE: &str =
    r#"window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  } 
  
  for (const type of ['chrome', 'node', 'electron']) {
    replaceText(`${type}-version`, process.versions[type])
  }
})"#;
