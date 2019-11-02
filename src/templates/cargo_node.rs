pub const DEFAULT_INDEX_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    {{ style? }}
    <script>
        var Module = {};
        var __cargo_web = {};
        Object.defineProperty( Module, 'canvas', {
            get: function() {
                if( __cargo_web.canvas ) {
                    return __cargo_web.canvas;
                }
                var canvas = document.createElement( 'canvas' );
                // document.querySelector( 'body' ).appendChild( canvas );
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

pub const BROWSER_INDEX_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    {{ style? }}
    <script>
        var module = {};
        var Module = {};
        var __cargo_web = {};
        Object.defineProperty( Module, 'canvas', {
            get: function() {
                if( __cargo_web.canvas ) {
                    return __cargo_web.canvas;
                }
                var canvas = document.createElement( 'canvas' );
                // document.querySelector( 'body' ).appendChild( canvas );
                __cargo_web.canvas = canvas;
                return canvas;
            }
        });
    </script>
</head>
<body>
    <script src="{{ name: str }}.wasm.js"></script>
    <script src="{{ name: str }}.js"></script>
</body>
</html>"#;

pub const CARGO_WEB_BROWSER_JS: &str = r#""use strict";

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
        var buffer = loadWebAssembly().buffer;

        WebAssembly.compile(buffer).then(function (mod) {
            var wasm_instance = WebAssembly.instantiate(mod, instance.imports);
            Rust.{{ name: str}} = wasm_instance
                .then(function (wasm_instance) {
                    var exports = instance.initialize(wasm_instance);
                    console.log("Finished loading Rust wasm module '{{ name: str}}'");
                    return exports;
                })
                .catch(function (error) {
                    console.log("Error loading Rust wasm module '{{ name: str}}':", error);
                    throw error;
                });
        }).catch(function (error) {
            console.log(error)
        });
    }(function () {
        {{ std_web: str }}
            }
        };
    }
    ));
}));"#;
