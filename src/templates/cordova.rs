pub const CORDOVA_PACKAGE_JSON_TEMPLATE: &str = r#"{
  "name": "{{ name: str }}",
  "displayName": "{{ name: str }}",
  "version": "1.0.0",
  "description": "Build by cargo-node.",
  "main": "index.js",
  "keywords": [
    "ecosystem:cordova",
    "Rust",
    "Wasm"
  ],
  "dependencies": {
    "cordova-android": "^8.0.0",
    "cordova-browser": "^6.0.0",
    "cordova-ios": "^5.0.1"
  },
  "devDependencies": {
    "cordova-plugin-whitelist": "^1.3.4"
  },
  "cordova": {
    "plugins": {
      "cordova-plugin-whitelist": {}
    },
    "platforms": [
      "ios",
      "android"
    ]
  }
}"#;

pub const CORDOVA_CONFIG_XML_TEMPLATE: &str = r#"<?xml version='1.0' encoding='utf-8'?>
<widget id="io.cordova.hellocordova" version="1.0.0" xmlns="http://www.w3.org/ns/widgets" xmlns:cdv="http://cordova.apache.org/ns/1.0">
    <name>{{ name: str }}</name>
    <description>
        Build with cargo-node.
    </description>
    <content src="index.html" />
    <plugin name="cordova-plugin-whitelist" spec="1" />
    <access origin="*" />
    <allow-intent href="http://*/*" />
    <allow-intent href="https://*/*" />
    <allow-intent href="tel:*" />
    <allow-intent href="sms:*" />
    <allow-intent href="mailto:*" />
    <allow-intent href="geo:*" />
    <platform name="android">
        <allow-intent href="market:*" />
        <access origin="file:///android_assets/www/" />
    </platform>
    <platform name="ios">
        <allow-intent href="itms:*" />
        <allow-intent href="itms-apps:*" />
    </platform>
</widget>"#;

pub const CORDOVA_COMPILE_WASM_JS_TEMPLATE: &str = r#"onmessage = function (e) {
    WebAssembly.compile(e.data).then(function(mod) { 
        this.postMessage(mod);
    }).catch(function (error) {
        console.log(error)
    })
}"#;

pub const CORDOVA_ANDROID_JS: &str = r#""use strict";

if (typeof Rust === "undefined") {
    var Rust = {};
}

(function (root, factory) {
    if (typeof define === "function" && define.amd) {
        define([], factory);
    } else if (typeof module === "object" && module.exports) {
        factory();
    } else {
        factory();
    }
}(this, function () {
    return (function (module_factory) {
        var instance = module_factory();
        const load = new Worker("compile_wasm.js");
        var wasm_buffer = loadWebAssembly().buffer;
        load.postMessage(wasm_buffer);
        load.onmessage = function (e) {
            var wasm_instance = WebAssembly.instantiate(e.data, instance.imports);

            Rust.{{ name: str }} = wasm_instance
                .then(function (wasm_instance) {
                    var exports = instance.initialize(wasm_instance);
                    console.log("Finished loading Rust wasm module '{{ name: str }}'");
                    return exports;
                })
                .catch(function (error) {
                    console.log("Error loading Rust wasm module '{{ name: str }}':", error);
                    throw error;
                });
        }
    }(function () {
        {{ std_web: str }}
            }
        };
    }
    ));
}));"#;
