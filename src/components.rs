use macroquad::{
    math::{Rect, Vec2},
    prelude::animation::AnimatedSprite,
    texture::Texture2D,
};
use rand::Rng;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Sprite {
    pub texture: Texture2D,
    pub source_rect: Option<Rect>, // none will render the entire sprite sheet
    pub animation: Option<AnimatedSprite>, // animated or static sprite
    pub dest_size: Option<Vec2>,
    pub flipped: bool,
    pub last_animation: usize,
}

#[derive(Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct Collider {
    pub collision_offset: Vec2, // feet collision offset -> for object collisions
    pub collision_size: Vec2,
    pub sprite_padding: Vec2,
    pub visible_size: Vec2,
}

pub struct Player {
    pub walk_speed: f32,
    pub attacking: bool,
    pub attack_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            walk_speed: 128.,
            attacking: false,
            attack_timer: 0.3,
        }
    }
}

impl Player {
    pub fn handle_player_animation(
        &self,
        velocity: &Velocity,
        sprite: &mut Sprite,
        player: &Player,
    ) {
        if let Some(ref mut anim) = sprite.animation {
            // movement
            if velocity.x != 0. || velocity.y != 0. {
                let animation_index = match (velocity.x, velocity.y) {
                    (x, _) if x > 0. => {
                        sprite.flipped = false;
                        2
                    } // right
                    (x, _) if x < 0. => {
                        sprite.flipped = true;
                        2
                    } // left
                    (_, y) if y < 0. => {
                        sprite.flipped = false;
                        4
                    } // up
                    (_, y) if y > 0. => {
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

            if player.attacking {
                anim.set_animation(match sprite.last_animation {
                    2 => 7,
                    4 => 8,
                    1 => 7,
                    _ => 0,
                });
            }

            anim.update();
        }
    }
}

pub enum AIType {
    Patrol,
    ChasePlayer,
    Wander,
}

pub struct Enemy {
    pub walk_speed: f32,
    pub movement_timer: f32,
    pub ai_type: AIType,
    pub change_direction_interval: f32,

    pub attacking: bool,
    pub attack_timer: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            walk_speed: 16.,
            movement_timer: 0.,
            ai_type: AIType::Wander,
            change_direction_interval: 1.5,

            attacking: false,
            attack_timer: 0.3,
        }
    }
}

impl Enemy {
    pub fn change_direction(&self, velocity: &mut Velocity, walk_speed: f32) {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);

        // 360 degree movement
        velocity.x = angle.cos() * walk_speed;
        velocity.y = angle.sin() * walk_speed;
    }

    pub fn handle_enemy_animation(&self, velocity: &Velocity, sprite: &mut Sprite, enemy: &Enemy) {
        if let Some(ref mut anim) = sprite.animation {
            if velocity.x != 0. || velocity.y != 0. {
                let animation_index = match (velocity.x, velocity.y) {
                    (x, _) if x > 0. => {
                        sprite.flipped = false;
                        4
                    } // right
                    (x, _) if x < 0. => {
                        sprite.flipped = true;
                        4
                    } // left
                    (_, y) if y < 0. => {
                        sprite.flipped = false;
                        5
                    } // up
                    (_, y) if y > 0. => {
                        sprite.flipped = false;
                        3
                    } // down
                    _ => sprite.last_animation,
                };
                sprite.last_animation = animation_index;
                anim.set_animation(animation_index);
            } else {
                let idle_animation = match sprite.last_animation {
                    4 => 1, // right -> idle_right
                    5 => 2, // up -> up_idle
                    _ => 0, // default idle
                };
                anim.set_animation(idle_animation);
            }
        }
    }
}
