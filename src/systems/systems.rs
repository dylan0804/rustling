use macroquad::{
    camera::set_camera,
    color::{RED, WHITE},
    input::{is_key_down, KeyCode},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
    time::get_frame_time,
};
use macroquad_tiled::Map;

use crate::{
    components::{
        collider::Collider,
        direction::Direction,
        enemy::{AIType, Enemy},
        player::{self, Player},
        position::Position,
        sprite::Sprite,
        velocity::Velocity,
    },
    resources::Resources,
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
    for (sprite, velocity, enemy) in world.query::<(&mut Sprite, &Velocity, &mut Enemy)>() {
        enemy.handle_enemy_animation(velocity, sprite);
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
            velocity.y = -player.walk_speed;
            player.last_direction = Direction::Up;
        }
        if is_key_down(KeyCode::Down) {
            velocity.y = player.walk_speed;
            player.last_direction = Direction::Down;
        }
        if is_key_down(KeyCode::Left) {
            velocity.x = -player.walk_speed;
            player.last_direction = Direction::Left;
        }
        if is_key_down(KeyCode::Right) {
            velocity.x = player.walk_speed;
            player.last_direction = Direction::Right;
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

pub fn enemy_aggro_system(world: &mut World) {
    let player_pos =
        if let Some((position, _)) = world.query::<(&Position, &Player)>().iter().next() {
            Vec2::new(position.x, position.y)
        } else {
            return;
        };

    for (enemy_pos, enemy) in world.query::<(&Position, &mut Enemy)>() {
        let enemy_pos_vec = Vec2::new(enemy_pos.x, enemy_pos.y);
        if !matches!(enemy.ai_type, AIType::Dead) {
            if enemy.should_attack_player(enemy_pos_vec, player_pos, enemy.attack_range) {
                enemy.ai_type = AIType::Attack;
            } else if enemy.should_chase_player(enemy_pos_vec, player_pos, enemy.aggro_range) {
                enemy.ai_type = AIType::ChasePlayer;
            } else {
                enemy.ai_type = AIType::Wander;
            }
        }
    }
}

pub fn enemy_movement_systems(world: &mut World) {
    let player_pos =
        if let Some((position, _)) = world.query::<(&Position, &Player)>().iter().next() {
            Vec2::new(position.x + 23., position.y + 29.)
        } else {
            return;
        };

    let dt = get_frame_time();

    for (enemy_pos, velocity, enemy) in world.query::<(&Position, &mut Velocity, &mut Enemy)>() {
        // + 12 to account for sprite padding
        let enemy_position = Vec2::new(enemy_pos.x + 12., enemy_pos.y + 12.);
        let direction = (player_pos - enemy_position).normalize();

        match enemy.ai_type {
            AIType::Wander => {
                enemy.attacking = false;
                enemy.movement_timer += dt;

                if enemy.movement_timer >= enemy.change_direction_interval {
                    enemy.movement_timer = 0.;
                    enemy.change_direction(velocity, enemy.walk_speed);
                }
            }
            AIType::ChasePlayer => {
                enemy.attacking = false;
                velocity.x = direction.x * enemy.chase_speed;
                velocity.y = direction.y * enemy.chase_speed;
            }
            AIType::Attack => {
                enemy.attack_timer += dt;
                if enemy.attack_timer >= enemy.attack_interval {
                    enemy.attack_timer = 0.;
                    enemy.attacking = true;
                    enemy.attack_animation_timer = 0.;
                }

                if enemy.attacking {
                    enemy.attack_animation_timer += dt;

                    // Keep moving during the entire attack animation
                    velocity.x = direction.x * enemy.attack_speed;
                    velocity.y = direction.y * enemy.attack_speed;

                    if enemy.attack_animation_timer >= enemy.attack_animation_duration {
                        enemy.attacking = false;
                    }
                } else {
                    velocity.x = 0.;
                    velocity.y = 0.;
                }
            }
            AIType::Dead => {
                velocity.x = 0.;
                velocity.y = 0.;
            }
        }
    }
}

pub fn player_attack_system(world: &mut World) {
    let dt = get_frame_time();

    // u enemy cooldowns first
    for enemy in world.query::<&mut Enemy>() {
        if enemy.hit_cooldown > 0.0 {
            enemy.hit_cooldown -= dt;
        }
    }

    let (attack_rect, is_attacking) =
        if let Some((position, player)) = world.query::<(&Position, &Player)>().iter().next() {
            let attack_rect = match player.last_direction {
                Direction::Right => Rect::new(position.x + 30.0, position.y + 24.0, 15.0, 20.0),
                Direction::Left => Rect::new(position.x + 3.0, position.y + 24.0, 15.0, 20.0),
                Direction::Up => Rect::new(position.x + 14.0, position.y + 18.0, 20.0, 20.0),
                Direction::Down => Rect::new(position.x + 15.0, position.y + 36.0, 20.0, 15.0),
            };
            (attack_rect, player.attacking)
        } else {
            return;
        };

    if is_attacking {
        for (enemy_pos, enemy_collider, enemy) in
            world.query::<(&Position, &Collider, &mut Enemy)>()
        {
            if !matches!(enemy.ai_type, AIType::Dead) {
                let enemy_rect = Rect::new(
                    enemy_pos.x + 10.0,
                    enemy_pos.y + 10.0,
                    enemy_collider.collision_size.x,
                    enemy_collider.collision_size.y,
                );

                if attack_rect.overlaps(&enemy_rect) && enemy.hit_cooldown <= 0.0 {
                    enemy.ai_type = AIType::Dead;
                }
            }
        }
    }
}

fn is_player_hit(player_rect: &Rect, enemy_rect: &Rect) -> bool {
    player_rect.overlaps(&enemy_rect)
}

pub fn hit_systems(world: &mut World) {
    let dt = get_frame_time();

    for player in world.query::<&mut Player>() {
        if player.hit_cooldown_timer > 0.0 {
            player.hit_cooldown_timer -= dt;
        }
    }

    let mut player_was_hit = false;

    let (player_rect, cooldown_timer) =
        if let Some((position, player)) = world.query::<(&Position, &Player)>().iter().next() {
            (
                Rect::new(position.x + 18., position.y + 20., 13., 22.),
                player.hit_cooldown_timer,
            )
        } else {
            return;
        };

    for (position, collider, enemy) in world.query::<(&Position, &Collider, &Enemy)>() {
        if enemy.attacking && cooldown_timer <= 0.0 {
            let enemy_rect = Rect::new(
                position.x + 8.0,
                position.y + 8.0,
                collider.collision_size.x,
                collider.collision_size.y,
            );
            if is_player_hit(&player_rect, &enemy_rect) {
                player_was_hit = true;
                break;
            }
        }
    }

    if player_was_hit {
        for player in world.query::<&mut Player>() {
            player.hit_cooldown_timer = player.hit_cooldown_duration;
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
