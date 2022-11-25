use bevy::prelude::{Component, Entity};

#[derive(Debug, Component)]
pub struct AutoDespawn {
    pub entity: Entity,
    pub time_to_live: f32,
}
