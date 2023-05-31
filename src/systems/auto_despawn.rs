use super::spawn_explosion;
use bevy::prelude::*;

pub fn auto_despawn_system(
    mut commands: Commands,
    mut auto_despawners: Query<(&mut AutoDespawn, &Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut auto_despawner, transform, entity) in auto_despawners.iter_mut() {
        auto_despawner.time_to_live -= time.delta_seconds();
        if auto_despawner.time_to_live < 0.0 {
            commands.entity(entity).despawn_recursive();
            spawn_explosion(&mut commands, transform.translation);
        }
    }
}

#[derive(Debug, Component)]
pub struct AutoDespawn {
    pub time_to_live: f32,
}
