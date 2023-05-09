use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{AutoDespawn, GameAssets};

pub struct FireProjectileEvent {
    transform: Transform,
}

#[derive(Component, Default)]
pub struct Projectile {}

impl FireProjectileEvent {
    pub fn new(transform: &Transform) -> Self {
        Self {
            transform: transform.clone(),
        }
    }
}

pub fn fire_projectile(
    mut commands: Commands,
    mut event_reader: EventReader<FireProjectileEvent>,
    game_assets: Res<GameAssets>,
) {
    const PROJECTILE_FIRE_OFFSET: Vec3 = Vec3::new(0.0, 5.6, -3.5);
    const PROJECTILE_RADIUS: f32 = 0.2;
    const PROJECTILE_LIFETIME: f32 = 1.0;
    const PROJECTILE_VELOCITY: f32 = 30.0;

    for event in event_reader.iter() {
        let mut projectile = commands.spawn_empty();
        let entity_id = projectile.id();
        projectile.insert((
            SceneBundle {
                transform: event
                    .transform
                    .with_translation(event.transform * PROJECTILE_FIRE_OFFSET),
                scene: game_assets.projectile.clone(),
                ..Default::default()
            },
            Projectile::default(),
            RigidBody::KinematicVelocityBased,
            Velocity::linear(event.transform.forward() * PROJECTILE_VELOCITY),
            Collider::ball(PROJECTILE_RADIUS),
            AutoDespawn {
                entity: entity_id,
                time_to_live: PROJECTILE_LIFETIME,
            },
        ));
    }
}
