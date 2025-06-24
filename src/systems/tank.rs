use avian3d::prelude::*;
use bevy::{prelude::*, render::camera::Camera, window::PrimaryWindow};

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
    tanks: Query<
        &GlobalTransform,
        (
            With<Tank>,
            With<super::PlayerControllerConfiguration>,
            Without<Turret>,
        ),
    >,
    mut turrets: Query<(&mut Transform, &Turret), With<Turret>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
    let Ok(primary_window) = primary_windows.single() else {
        return;
    };

    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    // Convert cursor position to world ray
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Find intersection with ground plane (y = 0)
    let ground_y = 0.0;
    if ray.direction.y.abs() < f32::EPSILON {
        return; // Ray is parallel to ground
    }

    let t = (ground_y - ray.origin.y) / ray.direction.y;
    if t < 0.0 {
        return; // Intersection is behind the ray origin
    }

    let world_position = ray.origin + ray.direction * t;

    for (mut turret_transform, turret) in &mut turrets {
        // Get the tank's world position
        let Ok(tank_transform) = tanks.get(turret.tank) else {
            continue;
        };

        // Convert world target position to tank's local coordinate space
        let tank_transform_matrix = tank_transform.compute_matrix();
        let local_target = tank_transform_matrix
            .inverse()
            .transform_point3(world_position);

        // Create a target position at the same height as the turret (y=0 in local space)
        let local_target_position = Vec3::new(local_target.x, 0.0, local_target.z);

        // Use look_at to rotate the turret towards the target in local space
        turret_transform.look_at(local_target_position, Vec3::Y);
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
            SceneRoot(game_assets.get_asset(GameAssetName::Projectile)),
            transform
                .compute_transform()
                .with_translation(transform.transform_point(PROJECTILE_FIRE_OFFSET)),
            Projectile::default(),
            RigidBody::Kinematic,
            LinearVelocity::from(transform.forward() * PROJECTILE_VELOCITY),
            Collider::sphere(PROJECTILE_RADIUS),
            AutoDespawn::new(PROJECTILE_LIFETIME),
        ));
    }
}
