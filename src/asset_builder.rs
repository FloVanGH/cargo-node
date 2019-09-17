use std::{fs, path::Path};

use crate::NodeToml;

/// The `AssetBuilder` is used to create assets folders and copy assets to build and deploy directories.
pub struct AssetBuilder;

impl AssetBuilder {
    /// Creates a new `AssetBuilder`.
    pub fn new() -> Self {
        AssetBuilder
    }

    /// Create asset folders and copy assets.
    pub fn build(&self, app_name: &str, output_dir: &str, node_toml: &NodeToml) {
        if let Some(assets) = node_toml.assets(app_name) {
            println!("Copy assets.");
            let asset_dir = format!("{}/{}", output_dir, assets);

            fs::create_dir_all(asset_dir.clone()).unwrap();

            for entry in fs::read_dir(assets).unwrap() {
                let file_name = entry.unwrap().file_name().into_string().unwrap();
                let asset_file = format!("{}/{}", asset_dir, file_name);

                println!("\t{}", asset_file);

                fs::copy(format!("{}/{}", assets, file_name), asset_file).unwrap();
            }
        }

        if let Some(fonts) = node_toml.fonts(app_name) {
            println!("Copy fonts.");
            let font_dir = format!("{}/fonts", output_dir);
            fs::create_dir_all(font_dir.clone()).unwrap();

            for font in fonts {
                let path = Path::new(font.src.as_str());
                let font_file = format!(
                    "{}/{}",
                    font_dir,
                    path.file_name().unwrap().to_str().unwrap()
                );

                println!("\t{}", font_file);

                fs::copy(path, font_file).unwrap();
            }
        }
    }
}
