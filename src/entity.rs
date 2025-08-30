use crate::components::{Enemy, Player};

pub type Entity = usize;

pub enum EntityType {
    Player,
    Enemy,
}
