use macroquad::{
    camera::set_camera,
    color::{RED, WHITE},
    input::{is_key_down, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use macroquad_tiled::Map;

use crate::{
    components::{Collider, Controllable, Player, Position, Sprite, Velocity},
    resources::Resources,
    world::{self, World},
};

pub fn render_systems(world: &mut World) {
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
                flip_x: sprite.flipped,
                ..Default::default()
            },
        );
    }
}

pub fn tilemap_render_system(tiled_map: &Map) {
    // println!(
    //     "Map size: {}x{}",
    //     tiled_map.raw_tiled_map.width, tiled_map.raw_tiled_map.height
    // );
    // println!(
    //     "Tile size: {}x{}",
    //     tiled_map.raw_tiled_map.tilewidth, tiled_map.raw_tiled_map.tileheight
    // );
    tiled_map.draw_tiles("background", Rect::new(0.0, 0.0, 960.0, 960.0), None);
    tiled_map.draw_tiles("decorations", Rect::new(0.0, 0.0, 960.0, 960.0), None);
}

pub fn animation_systems(world: &mut World) {
    // update moving animation
    for (sprite, velocity) in world.query::<(&mut Sprite, &Velocity)>() {
        if let Some(ref mut anim) = sprite.animation {
            if velocity.x != 0.0 || velocity.y != 0.0 {
                let animation_index = match (velocity.x, velocity.y) {
                    (x, _) if x > 0.0 => {
                        sprite.flipped = false;
                        2
                    } // right
                    (x, _) if x < 0.0 => {
                        sprite.flipped = true;
                        2
                    } // left
                    (_, y) if y < 0.0 => {
                        sprite.flipped = false;
                        4
                    } // up
                    (_, y) if y > 0.0 => {
                        sprite.flipped = false;
                        1
                    } // down
                    _ => sprite.last_animation,
                };
                sprite.last_animation = animation_index;
                anim.set_animation(animation_index);
            } else {
                let idle_animation = match sprite.last_animation {
                    2 => 3, // right -> idle_right
                    4 => 5, // up -> up_idle
                    _ => 0, // default idle
                };
                anim.set_animation(idle_animation);
            }
            anim.update();
        }
    }

    // update unmoving entities
    for sprite in world.query::<&mut Sprite>() {
        if let Some(ref mut anim) = sprite.animation {
            anim.update();
        }
    }
}

pub fn input_systems(world: &mut World) {
    for (velocity, controllable) in world.query::<(&mut Velocity, &Controllable)>() {
        velocity.x = 0.;
        velocity.y = 0.;

        if is_key_down(KeyCode::Up) {
            velocity.y = -controllable.walk_speed
        }
        if is_key_down(KeyCode::Down) {
            velocity.y = controllable.walk_speed
        }
        if is_key_down(KeyCode::Left) {
            velocity.x = -controllable.walk_speed
        }
        if is_key_down(KeyCode::Right) {
            velocity.x = controllable.walk_speed
        }

        // normalize diagonal movement
        let length = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
        if length > 0.0 {
            velocity.x = (velocity.x / length) * controllable.walk_speed;
            velocity.y = (velocity.y / length) * controllable.walk_speed;
        }
    }
}

pub fn movement_systems(world: &mut World, map: &Map) {
    let dt = get_frame_time();
    for (position, velocity, collider) in world.query::<(&mut Position, &Velocity, &Collider)>() {
        let new_pos = Vec2::new(position.x + velocity.x * dt, position.y + velocity.y * dt);
        let collision_box = Rect::new(
            new_pos.x + collider.collision_offset.x,
            new_pos.y + collider.collision_offset.y,
            collider.collision_size.x,
            collider.collision_size.y,
        );

        if !check_collision_with_objects(collision_box, &map) {
            let clamped_x =
                (new_pos.x + collider.sprite_padding.x).clamp(0.0, 960.0 - collider.visible_size.x);
            let clamped_y =
                (new_pos.y + collider.sprite_padding.y).clamp(0.0, 960.0 - collider.visible_size.y);

            position.x = clamped_x - collider.sprite_padding.x;
            position.y = clamped_y - collider.sprite_padding.y;
        }
    }
}

pub fn check_collision_with_objects(player_box: Rect, map: &Map) -> bool {
    if let Some(collision_layer) = map.layers.get("collisions") {
        for object in &collision_layer.objects {
            let object_rect = Rect::new(
                object.world_x,
                object.world_y,
                object.world_w,
                object.world_h,
            );
            if player_box.overlaps(&object_rect) {
                return true;
            }
        }
    }
    false
}

pub fn camera_systems(world: &mut World, resources: &mut Resources) {
    for (position, _) in world.query::<(&Position, &Player)>() {
        let target_x = position.x + 24.0; // center on player
        let target_y = position.y + 24.0;

        // // World dimensions (60x60 tiles * 16 pixels per tile = 960x960)
        // let world_width = 960.0;
        // let world_height = 960.0;
        //
        // // Calculate how much of the world the camera can see
        // let zoom_x = resources.camera.zoom.x;
        // let zoom_y = resources.camera.zoom.y;
        //
        // // Convert zoom back to viewport size in world units
        // // zoom = desired_world_units / screen_pixels
        // // so: desired_world_units = zoom * screen_pixels
        // let viewport_width = 0.25 * screen_width();
        // let viewport_height = 0.333 * screen_height();
        // // println!("{} {}", viewport_width, viewport_height);
        //
        // // Calculate clamping bounds to prevent showing black area
        // // Camera target should never be closer to edge than half viewport
        // let min_target_x = viewport_width / 2.0;
        // let max_target_x = world_width - viewport_width / 2.0;
        // let min_target_y = viewport_height / 2.0;
        // let max_target_y = world_height - viewport_height / 2.0;
        //
        // Clamp camera target
        resources.camera.target = Vec2::new(target_x, target_y);

        set_camera(&resources.camera);
        return; // player only
    }
}
