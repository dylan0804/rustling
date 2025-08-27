use crate::{
    components::{Collider, Controllable, Player, Position, Sprite, Velocity},
    world::{self, World},
};

pub trait ComponentQuery<'a> {
    type Output; // implementor needs to specify their return values
    fn find_entities(world: &'a mut World) -> Vec<Self::Output>;
}

impl<'a> ComponentQuery<'a> for (&'a Sprite, &'a Position) {
    type Output = (&'a Sprite, &'a Position);

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();
        for entity in 0..world.entities_count {
            if let (Some(sprite), Some(position)) = (
                world.get_component::<Sprite>(entity),
                world.get_component::<Position>(entity),
            ) {
                entities.push((sprite, position));
            }
        }
        entities
    }
}

impl<'a> ComponentQuery<'a> for (&'a mut Velocity, &'a Controllable) {
    type Output = (&'a mut Velocity, &'a Controllable);

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();
        unsafe {
            let world_ptr = &raw mut *world;

            for entity in 0..(*world_ptr).entities_count {
                if let (Some(velocity), Some(controllable)) = (
                    (*world_ptr).get_component_mut::<Velocity>(entity),
                    (*world_ptr).get_component::<Controllable>(entity),
                ) {
                    entities.push((velocity, controllable));
                }
            }
        }
        entities
    }
}

impl<'a> ComponentQuery<'a> for (&'a mut Sprite, &'a Velocity) {
    type Output = (&'a mut Sprite, &'a Velocity);

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();

        unsafe {
            let world_ptr = &raw mut *world;
            for entity in 0..(*world_ptr).entities_count {
                if let (Some(sprite), Some(velocity)) = (
                    (*world_ptr).get_component_mut::<Sprite>(entity),
                    (*world_ptr).get_component::<Velocity>(entity),
                ) {
                    entities.push((sprite, velocity));
                }
            }
        }

        entities
    }
}

impl<'a> ComponentQuery<'a> for (&'a mut Position, &'a Velocity, &'a Collider) {
    type Output = (&'a mut Position, &'a Velocity, &'a Collider);

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();
        unsafe {
            let world_ptr = &raw mut *world;

            for entity in 0..(*world_ptr).entities_count {
                if let (Some(position), Some(velocity), Some(collider)) = (
                    (*world_ptr).get_component_mut::<Position>(entity),
                    (*world_ptr).get_component::<Velocity>(entity),
                    (*world_ptr).get_component::<Collider>(entity),
                ) {
                    entities.push((position, velocity, collider));
                }
            }
        }
        entities
    }
}

impl<'a> ComponentQuery<'a> for (&'a Position, &'a Player) {
    type Output = (&'a Position, &'a Player);

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();
        for entity in 0..world.entities_count {
            if let (Some(position), Some(player)) = (
                world.get_component::<Position>(entity),
                world.get_component::<Player>(entity),
            ) {
                entities.push((position, player));
            }
        }
        entities
    }
}

impl<'a> ComponentQuery<'a> for &'a mut Sprite {
    type Output = &'a mut Sprite;

    fn find_entities(world: &'a mut World) -> Vec<Self::Output> {
        let mut entities = Vec::new();

        unsafe {
            let world_ptr = &raw mut *world;
            for entity in 0..(*world_ptr).entities_count {
                if let Some(sprite) = (*world_ptr).get_component_mut::<Sprite>(entity) {
                    entities.push(sprite);
                }
            }
        }

        entities
    }
}
