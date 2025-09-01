use std::error::Error;

use macroquad::prelude::*;

use crate::{resources::Resources, world::World};

pub mod components;
pub mod entity;
pub mod query;
pub mod resources;
pub mod systems;
pub mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rustling".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    match run().await {
        Ok(_) => println!("Game running"),
        Err(e) => println!("error: {:?}", e),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();
    let mut resources = Resources::load_all(&mut world).await?;

    loop {
        clear_background(BLANK);

        systems::tilemap_render_system(&resources.tiled_map, &mut world);

        systems::animation_systems(&mut world);
        systems::input_systems(&mut world);
        systems::enemy_aggro_system(&mut world);
        systems::player_attack_system(&mut world);

        systems::enemy_movement_systems(&mut world);
        systems::hit_systems(&mut world);
        systems::movement_systems(&mut world, &resources.tiled_map);
        systems::camera_systems(&mut world, &mut resources);
        next_frame().await;
    }
}
