use macroquad::math::Vec2;

pub struct Collider {
    pub collision_offset: Vec2, // feet collision offset -> for object collisions
    pub collision_size: Vec2,
    pub sprite_padding: Vec2,
    pub visible_size: Vec2,
}
