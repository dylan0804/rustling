use crate::components::{direction::Direction, sprite::Sprite, velocity::Velocity};

pub struct Player {
    pub walk_speed: f32,
    pub attacking: bool,
    pub attack_timer: f32,

    pub hit_cooldown_duration: f32, // iframes
    pub hit_cooldown_timer: f32,
    pub last_direction: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            walk_speed: 128.,
            attacking: false,
            attack_timer: 0.3,
            hit_cooldown_duration: 1.,
            hit_cooldown_timer: 0.,

            last_direction: Direction::Down,
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
                    1 => 6,
                    _ => 0,
                });
            }

            anim.update();
        }
    }
}
