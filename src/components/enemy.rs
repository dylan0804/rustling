use crate::components::{sprite::Sprite, velocity::Velocity};
use macroquad::{
    math::Vec2,
    prelude::animation::{AnimatedSprite, Animation},
};
use rand::prelude::*;

#[derive(Debug)]
pub enum AIType {
    Attack,
    ChasePlayer,
    Wander,
    Dead,
}

pub struct Enemy {
    pub walk_speed: f32,
    pub chase_speed: f32,
    pub attack_speed: f32,
    pub attack_interval: f32,
    pub movement_timer: f32,
    pub ai_type: AIType,
    pub change_direction_interval: f32,

    pub attacking: bool,
    pub attack_timer: f32,
    pub aggro_range: f32,
    pub attack_range: f32,
    pub attack_animation_timer: f32,
    pub attack_animation_duration: f32,
    pub hit_cooldown: f32,
    pub death_animation_finished: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            walk_speed: 24.,
            chase_speed: 48.,
            attack_speed: 96.,
            movement_timer: 0.,
            ai_type: AIType::Wander,
            change_direction_interval: 1.5,
            attack_interval: 1.5,
            attacking: false,
            attack_timer: 1.5,
            aggro_range: 100.,
            attack_range: 60.,
            attack_animation_timer: 0.3,
            attack_animation_duration: 0.3,
            hit_cooldown: 0.,
            death_animation_finished: false,
        }
    }
}

impl Enemy {
    pub fn change_direction(&self, velocity: &mut Velocity, walk_speed: f32) {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);

        // 50% chance to stop
        if rng.random_range(0.0..1.0) < 0.5 {
            velocity.x = 0.;
            velocity.y = 0.;
        } else {
            // 360 degree movement
            velocity.x = angle.cos() * walk_speed;
            velocity.y = angle.sin() * walk_speed;
        }
    }

    pub fn should_attack_player(
        &self,
        enemy_pos: Vec2,
        player_pos: Vec2,
        attack_range: f32,
    ) -> bool {
        let dx = enemy_pos.x - player_pos.x;
        let dy = enemy_pos.y - player_pos.y;

        let distance_squared = dx * dx + dy * dy;
        distance_squared <= attack_range * attack_range
    }

    pub fn should_chase_player(&self, enemy_pos: Vec2, player_pos: Vec2, aggro_range: f32) -> bool {
        let dx = enemy_pos.x - player_pos.x;
        let dy = enemy_pos.y - player_pos.y;

        let distance_squared = dx * dx + dy * dy;
        distance_squared <= aggro_range * aggro_range
    }

    pub fn handle_enemy_animation(&mut self, velocity: &Velocity, sprite: &mut Sprite) {
        if let Some(ref mut anim) = sprite.animation {
            if matches!(self.ai_type, AIType::Dead) {
                if !self.death_animation_finished {
                    anim.set_animation(10);

                    if anim.is_last_frame() {
                        anim.set_frame(4); // reset to first frame of death animation
                        anim.update();
                        self.death_animation_finished = true;
                        anim.playing = false;
                    }
                }
            } else if velocity.x != 0. || velocity.y != 0. {
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
                anim.update();
            } else {
                let idle_animation = match sprite.last_animation {
                    4 => 1, // right -> idle_right
                    5 => 2, // up -> up_idle
                    _ => 0, // default idle
                };
                anim.set_animation(idle_animation);
                anim.update();
            }

            if !matches!(self.ai_type, AIType::Dead) && self.attacking {
                anim.set_animation(match sprite.last_animation {
                    3 => 6,
                    4 => 7,
                    5 => 8,
                    _ => 0,
                });
                anim.update();
            }
        }
    }
}

pub fn animated_skeleton() -> Option<AnimatedSprite> {
    Some(AnimatedSprite::new(
        48,
        48,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "idle_sides".to_string(),
                row: 1,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "idle_up".to_string(),
                row: 2,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "move_down".to_string(),
                row: 3,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "move_sides".to_string(),
                row: 4,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "move_up".to_string(),
                row: 5,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "attack_down".to_string(),
                row: 6,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "attack_sides".to_string(),
                row: 7,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "attack_up".to_string(),
                row: 8,
                frames: 6,
                fps: 4,
            },
            Animation {
                name: "damaged_down".to_string(),
                row: 9,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "death".to_string(),
                row: 12,
                frames: 5,
                fps: 8,
            },
        ],
        true,
    ))
}

pub fn animated_slime() -> Option<AnimatedSprite> {
    Some(AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "idle_sides".to_string(),
                row: 1,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "idle_up".to_string(),
                row: 2,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "move_down".to_string(),
                row: 3,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "move_sides".to_string(),
                row: 4,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "move_up".to_string(),
                row: 5,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "attack_down".to_string(),
                row: 6,
                frames: 7,
                fps: 4,
            },
            Animation {
                name: "attack_sides".to_string(),
                row: 7,
                frames: 7,
                fps: 4,
            },
            Animation {
                name: "attack_up".to_string(),
                row: 8,
                frames: 7,
                fps: 4,
            },
            Animation {
                name: "damaged_down".to_string(),
                row: 9,
                frames: 4,
                fps: 4,
            },
            Animation {
                name: "death".to_string(),
                row: 12,
                frames: 5,
                fps: 8,
            },
        ],
        true,
    ))
}
