use std::fs::File;

use macroquad::{
    color::WHITE,
    math::Rect,
    prelude::collections::storage,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{screen_height, screen_width},
};

use crate::{
    components::{Position, Sprite},
    resources::Resources,
    world::World,
};

pub fn render_systems(world: &World) {
    for (sprite, position) in world.query::<(&Sprite, &Position)>() {
        let source = if let Some(anim) = &sprite.animation {
            Some(anim.frame().source_rect) // animated
        } else {
            sprite.source_rect // static
        };

        draw_texture_ex(
            &sprite.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                source,
                dest_size: sprite.dest_size,
                ..Default::default()
            },
        );
    }
}

pub fn tilemap_render_system() {
    let resources = storage::get::<Resources>();
    resources.tiled_map.draw_tiles(
        "background",
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        None,
    );
    resources.tiled_map.draw_tiles(
        "decorations",
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        None,
    );
}

pub fn animation_systems(world: &mut World) {
    for sprite in world.query_mut::<Sprite>() {
        if let Some(ref mut anim) = sprite.animation {
            anim.update();
        }
    }
}
