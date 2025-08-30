use macroquad::{
    camera::set_camera,
    color::{RED, WHITE},
    input::{is_key_down, KeyCode},
    math::{Rect, Vec2},
    prelude::animation::{AnimatedSprite, Animation},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    time::get_frame_time,
};
use macroquad_tiled::Map;

use crate::{
    components::{AIType, Collider, Enemy, Player, Position, Sprite, Velocity},
    entity::EntityType,
    resources::{self, Resources},
    world::{self, World, WORLD_HEIGHT, WORLD_WIDTH},
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

pub fn tilemap_render_system(tiled_map: &Map, world: &mut World) {
    tiled_map.draw_tiles(
        "background",
        Rect::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
        None,
    );
    tiled_map.draw_tiles(
        "decorations",
        Rect::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
        None,
    );
    tiled_map.draw_tiles(
        "decorations_2",
        Rect::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
        None,
    );

    // render order here
    render_systems(world);

    tiled_map.draw_tiles(
        "foreground",
        Rect::new(0.0, 0.0, WORLD_WIDTH, WORLD_HEIGHT),
        None,
    );
}

fn player_animation_system(world: &mut World) {
    for (sprite, velocity, player) in world.query::<(&mut Sprite, &Velocity, &Player)>() {
        player.handle_player_animation(velocity, sprite, player);
    }
}

fn enemy_animation_system(world: &mut World) {
    for (sprite, velocity, enemy) in world.query::<(&mut Sprite, &Velocity, &Enemy)>() {
        enemy.handle_enemy_animation(velocity, sprite, enemy);
    }
}

pub fn animation_systems(world: &mut World) {
    // update moving animation
    player_animation_system(world);
    enemy_animation_system(world);

    // update unmoving entities
    for sprite in world.query::<&mut Sprite>() {
        if let Some(ref mut anim) = sprite.animation {
            anim.update();
        }
    }
}

pub fn input_systems(world: &mut World) {
    for (velocity, player) in world.query::<(&mut Velocity, &mut Player)>() {
        velocity.x = 0.;
        velocity.y = 0.;

        // movement related keypresses
        if is_key_down(KeyCode::Up) {
            velocity.y = -player.walk_speed
        }
        if is_key_down(KeyCode::Down) {
            velocity.y = player.walk_speed
        }
        if is_key_down(KeyCode::Left) {
            velocity.x = -player.walk_speed
        }
        if is_key_down(KeyCode::Right) {
            velocity.x = player.walk_speed
        }

        // normalize diagonal movement
        let length = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
        if length > 0.0 {
            velocity.x = (velocity.x / length) * player.walk_speed;
            velocity.y = (velocity.y / length) * player.walk_speed;
        }

        // attack related kepresses
        if is_key_down(KeyCode::Z) {
            player.attacking = true;
            player.attack_timer = 0.3;
        }

        // countdown attack_timer
        if player.attack_timer >= 0.0 {
            player.attack_timer -= get_frame_time();
            if player.attack_timer <= 0.0 {
                player.attacking = false;
            }
        }
    }
}

pub fn enemy_movement_systems(world: &mut World) {
    let dt = get_frame_time();
    for (velocity, enemy) in world.query::<(&mut Velocity, &mut Enemy)>() {
        match enemy.ai_type {
            AIType::Wander => {
                enemy.movement_timer += dt;

                if enemy.movement_timer >= enemy.change_direction_interval {
                    println!("here");
                    enemy.movement_timer = 0.;
                    enemy.change_direction(velocity, enemy.walk_speed);
                }
            }
            _ => {}
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
            let clamped_x = (new_pos.x + collider.sprite_padding.x)
                .clamp(0.0, WORLD_WIDTH - collider.visible_size.x);
            let clamped_y = (new_pos.y + collider.sprite_padding.y)
                .clamp(0.0, WORLD_HEIGHT - collider.visible_size.y);

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

        let clamped_x = target_x.clamp(256.0, WORLD_WIDTH - 256.0);
        let clamped_y = target_y.clamp(144.0, WORLD_HEIGHT - 144.0);
        resources.camera.target = Vec2::new(clamped_x, clamped_y);

        set_camera(&resources.camera);
        return; // player only
    }
}
