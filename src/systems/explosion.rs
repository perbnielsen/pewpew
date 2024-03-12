use std::time::Duration;

use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Bundle)]
pub struct ExplosionBundle {
    transform: Transform,
    name: Name,
    global_transform: GlobalTransform,
    explosion: Explosion,
    collider: Collider,
}

impl ExplosionBundle {
    pub fn new(translation: Vec3, start: Duration) -> Self {
        const EXPLOSION_DURATION: f32 = 0.5;
        const EXPLOSION_RADIUS: f32 = 10.0;

        Self {
            transform: Transform {
                translation,
                ..Default::default()
            },
            global_transform: GlobalTransform::default(),
            explosion: Explosion {
                start,
                duration: Duration::from_secs_f32(EXPLOSION_DURATION),
                size: EXPLOSION_RADIUS,
            },
            collider: Collider::from(Sphere::new(1.0)),
            explosion: Explosion {
                start,
                duration: Duration::from_secs_f32(EXPLOSION_DURATION),
                size: EXPLOSION_RADIUS,
            },
            collider: Collider::from(Sphere::new(1.0)),
            name: Name::new("Explosion"),
        }
    }
}

pub fn spawn_explosion(commands: &mut Commands, translation: Vec3, start_time: Duration) {
pub fn spawn_explosion(commands: &mut Commands, translation: Vec3, start_time: Duration) {
    let mut explosion = commands.spawn_empty();
    explosion.insert(ExplosionBundle::new(translation, start_time));
    explosion.insert(ExplosionBundle::new(translation, start_time));
}

#[derive(Component)]
pub struct Explosion {
    pub start: Duration,
    pub duration: Duration,
    pub size: f32,
    pub start: Duration,
    pub duration: Duration,
    pub size: f32,
}

pub fn explosion_system(
    mut commands: Commands,
    mut colliders: Query<(&mut Transform, Entity, &Explosion)>,
    mut colliders: Query<(&mut Transform, Entity, &Explosion)>,
    time: Res<Time>,
) {
    for (mut transform, entity, explosion) in &mut colliders {
        let progress =
            (time.elapsed() - explosion.start).as_secs_f32() / explosion.duration.as_secs_f32();
        if progress >= 1.0 {
            commands.entity(entity).despawn_recursive();
    for (mut transform, entity, explosion) in &mut colliders {
        let progress =
            (time.elapsed() - explosion.start).as_secs_f32() / explosion.duration.as_secs_f32();
        if progress >= 1.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            let radius = progress * explosion.size;
            transform.scale = Vec3::ONE * radius;
            let radius = progress * explosion.size;
            transform.scale = Vec3::ONE * radius;
        }
    }
}
