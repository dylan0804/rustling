use std::{any::Any, collections::HashMap, error::Error};

use macroquad::{
    math::{Rect, Vec2},
    prelude::animation::{AnimatedSprite, Animation},
    texture::Texture2D,
};
use macroquad_tiled::Object;

use crate::{
    components::{
        collider::Collider,
        enemy::{animated_skeleton, animated_slime, Enemy},
        player::Player,
        position::Position,
        sprite::Sprite,
        velocity::Velocity,
    },
    entity::Entity,
    query::ComponentQuery,
    resources,
};

pub static WORLD_WIDTH: f32 = 960.0;
pub static WORLD_HEIGHT: f32 = 512.0;

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity_id: usize,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity_id: usize) -> Self {
        Self { world, entity_id }
    }

    pub fn with<ComponentType: 'static>(self, component: ComponentType) -> Self {
        self.world
            .add_component_to_entity(self.entity_id, component);
        self
    }
}

pub struct World {
    pub entities_count: Entity,
    components_vec: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            components_vec: Vec::new(),
        }
    }

    pub fn add_entity(&mut self) -> usize {
        let entity_id = self.entities_count;

        // increase capacity if components have been initialized
        for component_vec in self.components_vec.iter_mut() {
            component_vec.push_none();
        }

        self.entities_count += 1;
        entity_id
    }

    pub fn spawn_entity(&mut self) -> EntityBuilder {
        let entity_id = self.add_entity();
        EntityBuilder::new(self, entity_id)
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: Entity,
        component: ComponentType,
    ) {
        // iterate through components vec and find the component of the same type
        for component_vec in self.components_vec.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return;
            }
        }

        // if not found then create a new component vec
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        self.components_vec.push(Box::new(new_component_vec));
    }

    pub fn get_component<ComponentType: 'static>(&self, entity: Entity) -> Option<&ComponentType> {
        for component_vec in self.components_vec.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return component_vec[entity].as_ref();
            }
        }

        None
    }

    pub fn get_component_mut<ComponentType: 'static>(
        &mut self,
        entity: Entity,
    ) -> Option<&mut ComponentType> {
        for component_vec in self.components_vec.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                return component_vec[entity].as_mut();
            }
        }

        None
    }

    pub fn query<'a, T>(&'a mut self) -> Vec<T::Output>
    where
        T: ComponentQuery<'a>,
    {
        T::find_entities(self)
    }

    pub fn add_object(
        &mut self,
        object: &Object,
        core_assets: &HashMap<String, Texture2D>,
    ) -> Result<(), Box<dyn Error>> {
        let entity_id = self.add_entity();
        let properties = &object.properties;

        self.add_component_to_entity(
            entity_id,
            Position {
                x: object.world_x,
                y: object.world_y - properties["dest_size_y"].parse::<f32>()?,
            },
        );
        self.add_component_to_entity(
            entity_id,
            Sprite {
                texture: core_assets[&format!("images/core/{}.png", object.name)].clone(),
                source_rect: Some(Rect::new(
                    properties["source_rect_x"].parse::<f32>()?,
                    properties["source_rect_y"].parse::<f32>()?,
                    properties["source_rect_w"].parse::<f32>()?,
                    properties["source_rect_h"].parse::<f32>()?,
                )),
                dest_size: Some(Vec2::new(
                    properties["dest_size_x"].parse::<f32>()?,
                    properties["dest_size_y"].parse::<f32>()?,
                )),
                animation: Some(AnimatedSprite::new(
                    object.world_w as u32,
                    object.world_h as u32,
                    &[Animation {
                        name: object.name.to_string(),
                        row: properties["row"].parse::<u32>()?,
                        frames: properties["frames"].parse::<u32>()?,
                        fps: 8,
                    }],
                    true,
                )),
                flipped: false,
                last_animation: 0,
            },
        );

        Ok(())
    }

    pub async fn spawn_enemy(&mut self, x: f32, y: f32, enemy: &str) -> Result<(), Box<dyn Error>> {
        self.spawn_entity()
            .with(Sprite {
                texture: resources::load_and_set_filter(&format!("images/content/{}.png", enemy))
                    .await?,
                source_rect: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
                dest_size: match enemy {
                    "skeleton" => Some(Vec2::new(48.0, 48.0)),
                    _ => Some(Vec2::new(32.0, 32.0)),
                },
                animation: match enemy {
                    "skeleton" => animated_skeleton(),
                    _ => animated_slime(),
                },
                flipped: false,
                last_animation: 0,
            })
            .with(Position { x, y })
            .with(Collider {
                visible_size: match enemy {
                    "skeleton" => Vec2::new(32.0, 32.0),
                    _ => Vec2::new(16.0, 16.0),
                },
                sprite_padding: Vec2::new(8.0, 8.0),
                collision_offset: Vec2::new(24.0, 16.0),
                collision_size: Vec2::new(16.0, 16.0),
            })
            .with(match enemy {
                "skeleton" => Enemy {
                    attack_range: 5.,
                    ..Default::default()
                },
                _ => Enemy::default(),
            })
            .with(Velocity { x: 8.0, y: 8.0 });

        Ok(())
    }

    pub async fn spawn_player(&mut self, x: f32, y: f32) -> Result<(), Box<dyn Error>> {
        self.spawn_entity()
            .with(Sprite {
                texture: crate::resources::load_and_set_filter("images/content/player.png").await?,
                source_rect: Some(Rect::new(0.0, 0.0, 48.0, 48.0)),
                dest_size: Some(Vec2::new(48.0, 48.0)),
                animation: Some(AnimatedSprite::new(
                    48,
                    48,
                    &[
                        Animation {
                            // 0
                            name: "idle".to_string(),
                            row: 0,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 1
                            name: "down".to_string(),
                            row: 3,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 2
                            name: "right".to_string(),
                            row: 4,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 3
                            name: "idle_right".to_string(),
                            row: 1,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 4
                            name: "up".to_string(),
                            row: 5,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 5
                            name: "up_idle".to_string(),
                            row: 2,
                            frames: 6,
                            fps: 4,
                        },
                        Animation {
                            // 6
                            name: "attack_down".to_string(),
                            row: 6,
                            frames: 4,
                            fps: 4,
                        },
                        Animation {
                            // 7
                            name: "attack_sides".to_string(),
                            row: 7,
                            frames: 4,
                            fps: 4,
                        },
                        Animation {
                            // 8
                            name: "attack_up".to_string(),
                            row: 8,
                            frames: 4,
                            fps: 4,
                        },
                        Animation {
                            // 9
                            name: "death".to_string(),
                            row: 9,
                            frames: 3,
                            fps: 4,
                        },
                    ],
                    true,
                )),
                flipped: false,
                last_animation: 0,
            })
            .with(Position { x, y })
            .with(Velocity::default())
            .with(Collider {
                collision_offset: Vec2::new(17., 38.),
                collision_size: Vec2::new(14., 5.),
                sprite_padding: Vec2::new(18.0, 20.0),
                visible_size: Vec2::new(18.0, 26.0),
            })
            .with(Player::default());
        Ok(())
    }
}
