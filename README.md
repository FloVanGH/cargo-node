# cargo-node

[![Build and test](https://github.com/FloVanGH/cargo-node/workflows/Build%20and%20test/badge.svg)](https://github.com/FloVanGH/cargo-node/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A cargo sub command to build, run and deploy rust wasm applications as browser, electron or cordova app.

## Features

Currently you could use the following commands:

* Build your project:

```sh
cargo node build
```

* Run your project:

```sh
cargo node run
```

* Deploy your project:

```sh
cargo node deploy
```

You could use the additional flags on the command line:

* Define the target platform (electron | browser | android) default is electron:

```sh
cargo node run --target browser
```

* Switch to release build:

```sh
cargo node deploy --release
```

* Build, run or deploy an example:

```sh
cargo node run --example my_example
```


## Installation

```sh
cargo install cargo node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

## Node.toml

`cargo node` provides an optional configuration file which you can put next to `cargo`'s [`Cargo.toml`].

Example:

```toml
[[apps]]
# Name of the executable
name = "my app"
# Defines the window width of the electron window
width = 300
# Defines the height of the electron window
height = 100
# Path of the assets folder with images, fonts, ...
assets = "assets/"
# Add custom fonts to your project
 [[apps.fonts]]
    font_family = "My Font"
    src = "fonts/MyFont.ttf"
```

## Tools under the hood 

* cargo-web  (Apache-2.0): https://github.com/koute/cargo-web used to build your rust application as client-side web application
* npm (The Artistic License 2.0): https://github.com/npm/cli used to install and handle electron, electron-packer and cordova 
* electron (MIT): https://github.com/electron/electron used to build, run desktop applications for Linux, macOS and Windows
* electron-packager (BSD-2-Clause): https://github.com/electron/electron-packager used to deploy desktop applications for Linux, macOS and Windows
* cordova (Apache-2.0): https://github.com/apache/cordova-cli used to build, run mobile applications Android
* wasm2js (Apache-2.0): https://github.com/WebAssembly/binaryen used to convert the wasm file to javascript as workaround for wasm loading on Android

## License

Licensed under MIT license ([LICENSE](./LICENSE)).