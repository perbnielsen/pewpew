use super::spawn_explosion;
use bevy::prelude::*;

// TODO: This system does not make sense.
//       It should be merged into the functionality of the mine, as they are the only ones that should have a time out.
//       Also, it seems difficult to make this generic enough to be useful

pub fn auto_despawn_system(
    mut commands: Commands,
    mut auto_despawners: Query<(&mut AutoDespawn, &Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut auto_despawner, transform, entity) in &mut auto_despawners {
        auto_despawner.time_to_live -= time.delta_seconds();
        if auto_despawner.time_to_live < 0.0 {
            commands.entity(entity).despawn_recursive();
            spawn_explosion(&mut commands, transform.translation, time.elapsed());
        }
    }
}

#[derive(Debug, Component)]
pub struct AutoDespawn {
    time_to_live: f32,
}

impl AutoDespawn {
    pub fn new(time_to_live: f32) -> Self {
        Self { time_to_live }
    }
}
