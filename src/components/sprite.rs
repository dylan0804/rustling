use macroquad::{
    math::{Rect, Vec2},
    prelude::animation::AnimatedSprite,
    texture::Texture2D,
};

pub struct Sprite {
    pub texture: Texture2D,
    pub source_rect: Option<Rect>, // none will render the entire sprite sheet
    pub animation: Option<AnimatedSprite>, // animated or static sprite
    pub dest_size: Option<Vec2>,
    pub flipped: bool,
    pub last_animation: usize,
}
