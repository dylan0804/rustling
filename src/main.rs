use std::error::Error;

use macroquad::prelude::*;

use crate::{
    resources::Resources,
    systems::systems::{
        animation_systems, camera_systems, enemy_aggro_system, enemy_movement_systems, hit_systems,
        input_systems, movement_systems, player_attack_system, tilemap_render_system,
    },
    world::World,
};

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

        tilemap_render_system(&resources.tiled_map, &mut world);

        animation_systems(&mut world);
        input_systems(&mut world);
        enemy_aggro_system(&mut world);
        player_attack_system(&mut world);

        enemy_movement_systems(&mut world);
        hit_systems(&mut world);
        movement_systems(&mut world, &resources.tiled_map);
        camera_systems(&mut world, &mut resources);
        next_frame().await;
    }
}
