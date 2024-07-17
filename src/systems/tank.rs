use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_3d::prelude::*;

use crate::systems::{AutoDespawn, GameAssetName};

use super::GameAssets;

#[derive(Component)]
pub struct Turret {
    pub tank: Entity,
}

#[derive(Component)]
pub struct Tank {}

#[derive(Event)]
pub struct FireProjectileEvent {
    source: Entity,
}

#[derive(Component, Default)]
pub struct Projectile {}

impl FireProjectileEvent {
    pub fn new(source: Entity) -> Self {
        Self { source }
    }
}

pub fn aim_turret(
    tanks: Query<&Transform, (With<Tank>, Without<Turret>)>,
    mut turrets: Query<(&mut Transform, &Parent), With<Turret>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    // camera: Query<&Transform, With<Camera3d>>,
) {
    let Ok(primary_window) = primary_windows.get_single() else {
        return;
    };

    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(primary_window.width(), primary_window.height());
    let cursor_position = cursor_position - window_size / 2.0;
    let cursor_position = Vec3::new(cursor_position.y, 0.0, -cursor_position.x) / 10.0;

    for (mut turret_transform, parent) in &mut turrets {
        if let Ok(tank_transform) = tanks.get(parent.get()) {
            let target = tank_transform
                .compute_matrix()
                .inverse()
                .transform_point3(cursor_position);
            turret_transform.look_at(target, Vec3::Y);
        }
    }
}

pub fn fire_projectile(
    turrets: Query<(&GlobalTransform, &Turret)>,
    mut commands: Commands,
    mut event_reader: EventReader<FireProjectileEvent>,
    game_assets: Res<GameAssets>,
) {
    const PROJECTILE_FIRE_OFFSET: Vec3 = Vec3::new(0.0, 5.6, -4.5);
    const PROJECTILE_RADIUS: f32 = 0.5;
    const PROJECTILE_LIFETIME: f32 = 1.0;
    const PROJECTILE_VELOCITY: f32 = 15.0;

    for event in event_reader.read() {
        let Some((transform, _)) = turrets
            .iter()
            .find(|(_, turret)| turret.tank == event.source)
        else {
            continue;
        };

        commands.spawn((
            SceneBundle {
                transform: transform
                    .compute_transform()
                    .with_translation(transform.transform_point(PROJECTILE_FIRE_OFFSET)),
                scene: game_assets.get_asset(GameAssetName::Projectile),
                ..default()
            },
            Projectile::default(),
            RigidBody::Kinematic,
            LinearVelocity::from(transform.forward() * PROJECTILE_VELOCITY),
            Collider::sphere(PROJECTILE_RADIUS),
            AutoDespawn::new(PROJECTILE_LIFETIME),
        ));
    }
}
