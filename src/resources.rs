use std::error::Error;

use macroquad::{
    file::load_string,
    prelude::collections::storage,
    texture::{load_texture, FilterMode, Texture2D},
};
use macroquad_tiled::Map;

pub struct Resources {
    pub tiled_map: Map,
}

impl Resources {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let tiled_map = Self::load_map().await?;
        Ok(Self { tiled_map })
    }

    pub async fn load_all() -> Result<(), Box<dyn Error>> {
        let resources = Self::new().await?;
        storage::store(resources);
        Ok(())
    }

    async fn load_map() -> Result<Map, Box<dyn Error>> {
        let tiled_map_json = load_string("map.json").await?;
        let map = macroquad_tiled::load_map(
            &tiled_map_json,
            &[
                (
                    "images/decor.png",
                    load_and_set_filter("images/decor.png").await?,
                ),
                (
                    "images/grass.png",
                    load_and_set_filter("images/grass.png").await?,
                ),
                (
                    "images/objects.png",
                    load_and_set_filter("images/objects.png").await?,
                ),
                (
                    "images/plains.png",
                    load_and_set_filter("images/plains.png").await?,
                ),
                (
                    "images/water1.png",
                    load_and_set_filter("images/water1.png").await?,
                ),
                (
                    "images/rock_in_water.png",
                    load_and_set_filter("images/rock_in_water.png").await?,
                ),
                (
                    "images/water_decorations.png",
                    load_and_set_filter("images/water_decorations.png").await?,
                ),
            ],
            &[],
        )?;

        Ok(map)
    }

    // fn load_embedded_assets(path: &str) -> Result<HashMap<String, Texture2D>, Box<dyn Error>> {
    //     let mut textures_map = HashMap::new();
    //
    //     PROJECT_DIR.find(path)?.for_each(|entry| {
    //         let asset_path = entry.path().to_string_lossy().to_string();
    //         let loaded_texture = Self::load_texture(&asset_path);
    //         textures_map.insert(asset_path, loaded_texture);
    //     });
    //
    //     Ok(textures_map)
    // }
}

pub async fn load_and_set_filter(path: &str) -> Result<Texture2D, Box<dyn Error>> {
    let texture = load_texture(path).await?;
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}
