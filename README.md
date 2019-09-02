# cargo-node

[![Build status](https://gitlab.com/FloVanGL/cargo-node/badges/master/build.svg)](https://gitlab.com/FloVanGL/cargo-node/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A cargo sub command to build, run and deploy rust wasm applications as electron or cordova app.

## Features

tbd

## Installation

tdb

## Node.tml

tbd


## Tools under the hood 

* cargo-web (Apache-2.0): https://github.com/koute/cargo-web used to build your rust application as client-side web application
* npm (The Artistic License 2.0): https://github.com/npm/cli used to install and handle electron, electron-packer and cordova 
* electron (MIT): https://github.com/electron/electron used to build, run desktop applications for Linux, macOS and Windows
* electron-packager (BSD-2-Clause): https://github.com/electron/electron-packager used to deploy desktop applications for Linux, macOS and Windows
* cordova (Apache-2.0): https://github.com/apache/cordova-cli used to build, run mobile applications Android
* wasm2js (Apache-2.0): https://github.com/WebAssembly/binaryen used to convert the wasm file to javascript as workaround for wasm loading on Android

## License

Licensed under MIT license ([LICENSE](./LICENSE)).