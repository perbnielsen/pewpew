use bevy::prelude::*;

pub fn movement_update_system(
    mut moving_objects: Query<(&Moving, &mut Transform)>,
    time: Res<Time>,
) {
    for (movement, mut transform) in &mut moving_objects {
        let forward_velocity = transform.forward() * movement.velocity * time.delta_seconds();
        transform.translation += forward_velocity;
        transform.rotation *= Quat::from_rotation_y(movement.delta_yaw * time.delta_seconds());
    }
}

#[derive(Component, Default, Reflect)]
pub struct Moving {
    pub speed: f32,
    pub rotation_speed: f32,

    pub velocity: f32,
    pub delta_yaw: f32,
}

impl Moving {
    pub fn new(speed: f32, rotation_speed: f32) -> Self {
        Self {
            speed,
            rotation_speed,
            ..default()
        }
    }
}
