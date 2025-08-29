use macroquad::{
    math::{Rect, Vec2},
    prelude::animation::AnimatedSprite,
    texture::Texture2D,
};

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

pub struct Controllable {
    pub walk_speed: f32,
}

impl Default for Controllable {
    fn default() -> Self {
        Self { walk_speed: 128.0 }
    }
}

pub struct Collider {
    pub collision_offset: Vec2, // feet collision offset -> for object collisions
    pub collision_size: Vec2,
    pub sprite_padding: Vec2,
    pub visible_size: Vec2,
}

#[derive(Default)]
pub struct AttackState {
    pub attacking: bool,
    pub attack_timer: f32,
}

pub struct Player;
