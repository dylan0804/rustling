use std::{collections::HashMap, error::Error};

use include_dir::{include_dir, Dir};
use macroquad::{
    camera::Camera2D,
    file::load_string,
    math::{Rect, Vec2},
    prelude::{
        animation::{AnimatedSprite, Animation},
        collections::storage,
    },
    texture::{load_texture, FilterMode, Texture2D},
    window::{screen_height, screen_width},
};
use macroquad_tiled::Map;

use crate::{
    components::{Position, Sprite},
    resources,
    world::{self, World},
};

static CORE_ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub struct Resources {
    pub tiled_map: Map,
    pub camera: Camera2D,
    core_assets: HashMap<String, Texture2D>,
}

impl Resources {
    async fn new(world: &mut World) -> Result<Self, Box<dyn Error>> {
        let core_assets = Self::load_core_assets("images/core/*.png").await?;
        let tiled_map = Self::load_map(world, &core_assets).await?;
        let camera = Camera2D {
            zoom: Vec2::new(5.0 / screen_width(), 5.0 / screen_height()),
            ..Default::default()
        };
        Ok(Self {
            tiled_map,
            core_assets,
            camera,
        })
    }

    pub async fn load_all(world: &mut World) -> Result<Resources, Box<dyn Error>> {
        let resources = Self::new(world).await?;
        // storage::store(resources);
        Ok(resources)
    }

    async fn load_map(
        world: &mut World,
        core_assets: &HashMap<String, Texture2D>,
    ) -> Result<Map, Box<dyn Error>> {
        let tiled_map_json = load_string("map.json").await?;
        let map = macroquad_tiled::load_map(
            &tiled_map_json,
            &[
                (
                    "images/core/decor.png",
                    core_assets["images/core/decor.png"].clone(),
                ),
                (
                    "images/core/grass.png",
                    core_assets["images/core/grass.png"].clone(),
                ),
                (
                    "images/core/objects.png",
                    core_assets["images/core/objects.png"].clone(),
                ),
                (
                    "images/core/plains.png",
                    core_assets["images/core/plains.png"].clone(),
                ),
                (
                    "images/core/water-sheet.png",
                    core_assets["images/core/water-sheet.png"].clone(),
                ),
                (
                    "images/core/rock_in_water.png",
                    core_assets["images/core/rock_in_water.png"].clone(),
                ),
                (
                    "images/core/water_decorations.png",
                    core_assets["images/core/water_decorations.png"].clone(),
                ),
            ],
            &[],
        )?;

        // get object layer -> object layer is for decorations (animated sprite)
        let object_layer = map
            .layers
            .get("objects")
            .ok_or_else(|| "Layer 'objects' not found")?;

        // iterate through all the objects and add entity and components
        for object in &object_layer.objects {
            world.add_object(&object, &core_assets)?;
        }

        Ok(map)
    }

    async fn load_core_assets(path: &str) -> Result<HashMap<String, Texture2D>, Box<dyn Error>> {
        let mut textures_map = HashMap::new();

        for entry in CORE_ASSETS_DIR.find(path)? {
            let asset_path = entry.path().to_string_lossy().to_string();
            let loaded_texture = load_and_set_filter(&asset_path).await?;
            textures_map.insert(asset_path, loaded_texture);
        }

        Ok(textures_map)
    }
}

pub async fn load_and_set_filter(path: &str) -> Result<Texture2D, Box<dyn Error>> {
    let texture = load_texture(path).await?;
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}
