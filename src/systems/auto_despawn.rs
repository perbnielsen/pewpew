use bevy::prelude::*;

pub fn auto_despawn_system(
    mut commands: Commands,
    mut auto_despawners: Query<&mut AutoDespawn>,
    time: Res<Time>,
) {
    for mut auto_despawner in auto_despawners.iter_mut() {
        auto_despawner.time_to_live -= time.delta_seconds();
        if auto_despawner.time_to_live < 0.0 {
            commands.entity(auto_despawner.entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Component)]
pub struct AutoDespawn {
    pub entity: Entity,
    pub time_to_live: f32,
}
