use crate::{
    components::{Position, Sprite},
    world::World,
};

pub trait ComponentQuery<'a> {
    type Output; // implementor needs to specify their return values
    fn find_entities(world: &'a World) -> Vec<Self::Output>;
}

impl<'a> ComponentQuery<'a> for (&'a Sprite, &'a Position) {
    type Output = (&'a Sprite, &'a Position);

    fn find_entities(world: &'a World) -> Vec<(&'a Sprite, &'a Position)> {
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
