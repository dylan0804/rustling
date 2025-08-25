use macroquad::{
    math::{Rect, Vec2},
    prelude::animation::AnimatedSprite,
    texture::Texture2D,
};

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Sprite {
    pub sprite_id: u32,
    pub texture: Texture2D,
    pub source_rect: Option<Rect>, // none will render the entire sprite sheet
    pub animation: Option<AnimatedSprite>, // animated or static sprite
    pub dest_size: Option<Vec2>,
}
