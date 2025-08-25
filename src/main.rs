use std::error::Error;

use macroquad::prelude::{
    animation::{AnimatedSprite, Animation},
    *,
};

use crate::{
    components::{Position, Sprite},
    resources::Resources,
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
    Resources::load_all().await?;
    let mut world = World::new();

    let entity_id = world.add_entity();
    world.add_component_to_entity(
        entity_id,
        Sprite {
            sprite_id: 0,
            texture: resources::load_and_set_filter("images/player.png").await?,
            source_rect: Some(Rect::new(0.0, 0.0, 48.0, 48.0)),
            dest_size: Some(Vec2::new(96.0, 96.0)),
            animation: Some(AnimatedSprite::new(
                48,
                48,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 2,
                    fps: 4,
                }],
                true,
            )),
        },
    );
    world.add_component_to_entity(entity_id, Position { x: 32.0, y: 48.0 });

    loop {
        clear_background(DARKGREEN);

        // world.render_entities(&tiled_map);
        // draw_texture_ex(&tileset, x, y, color, params);``
        systems::tilemap_render_system();
        systems::animation_systems(&mut world);
        systems::render_systems(&world);
        next_frame().await;
    }
}
