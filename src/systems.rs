use crate::auto_despawn::AutoDespawn;
use crate::{
    moving::Moving, player_controller_configuration::PlayerControllerConfiguration, Projectile,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_control_system(
    mut players: Query<(
        &mut Moving,
        &PlayerControllerConfiguration,
        &Transform,
        &Children,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut movement, player_controller, transform, _children) in players.iter_mut() {
        movement.velocity = 0.0;
        movement.delta_yaw = 0.0;

        if keyboard_input.pressed(player_controller.keycode_left) {
            movement.delta_yaw += movement.rotation_speed;
        }
        if keyboard_input.pressed(player_controller.keycode_right) {
            movement.delta_yaw -= movement.rotation_speed;
        }
        if keyboard_input.pressed(player_controller.keycode_forward) {
            movement.velocity += movement.speed;
        }
        if keyboard_input.pressed(player_controller.keycode_reverse) {
            movement.velocity -= movement.speed;
        }
        if keyboard_input.just_pressed(player_controller.keycode_fire) {
            fire_bullet(transform, &mut commands, &mut meshes);
        }
        if keyboard_input.just_pressed(player_controller.keycode_lay_mine) {
            lay_mine(&mut commands, transform, &mut meshes);
        }
    }
}

fn lay_mine(commands: &mut Commands, transform: &Transform, meshes: &mut ResMut<Assets<Mesh>>) {
    const MINE_RADIUS: f32 = 1.0;
    let mut mine = commands.spawn_empty();
    mine.insert(Collider::ball(MINE_RADIUS));
    mine.insert(PbrBundle {
        transform: transform.clone(),
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: MINE_RADIUS,
            subdivisions: 3,
        })),
        ..Default::default()
    });
}

    const BULLET_FIRE_OFFSET: Vec3 = Vec3::new(0.0, 1.52, -3.51);
    let transform = transform.with_translation(*transform * BULLET_FIRE_OFFSET);
    let mut bullet = commands.spawn_empty();
    let id = bullet.id();
    bullet.insert((
        PbrBundle {
            transform,
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.2,
                subdivisions: 3,
            })),
            ..Default::default()
        },
        Projectile::default(),
        Moving {
            velocity: 20.0,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.5),
        AutoDespawn {
            entity: id,
            time_to_live: 1.0,
        },
    ));
}

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

pub fn movement_update_system(
    mut moving_objects: Query<(&Moving, &mut Transform)>,
    time: Res<Time>,
) {
    for (movement, mut transform) in moving_objects.iter_mut() {
        let forward_velocity = transform.forward() * movement.velocity * time.delta_seconds();
        transform.translation += forward_velocity;
        transform.rotation *= Quat::from_rotation_y(movement.delta_yaw * time.delta_seconds());
    }
}
