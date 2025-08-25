use std::any::Any;

use macroquad::{math::Rect, prelude::animation::AnimatedSprite};
use macroquad_tiled::Map;

use crate::{
    components::{Position, Size, Sprite},
    entity::Entity,
    query::ComponentQuery,
};

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

    pub fn query<'a, T>(&'a self) -> Vec<T::Output>
    where
        T: ComponentQuery<'a>,
    {
        T::find_entities(&self)
    }

    pub fn query_mut<ComponentType: 'static>(&mut self) -> Vec<&mut ComponentType> {
        let mut results = Vec::new();
        for component_vec in self.components_vec.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                for component_type in component_vec {
                    if let Some(component) = component_type {
                        results.push(component);
                    }
                }
                break;
            }
        }

        results
    }
}
