use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Turret {}

#[derive(Component)]
pub struct Tank {
    pub entity: Entity,
}
pub fn aim_turret(
    tanks: Query<&Transform, Without<Turret>>,
    mut turrets: Query<(&mut Transform, &Parent), With<Turret>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(primary_window) = primary_windows.get_single() else {
        return;
    };

    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };

    let window_size = Vec2::new(primary_window.width(), primary_window.height());
    let cursor_position = cursor_position - window_size / 2.0;
    let cursor_position = Vec3::new(-cursor_position.y, 0.0, -cursor_position.x) / 10.0;

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

// pub fn turret_fire(
//     mut turrets: Query<(&Transform, &Turret)>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut fire_projectile_event_writer: EventWriter<FireProjectileEvent>,
// ) {
//     for (transform, turret) in turrets.iter_mut() {
//         if keyboard_input.just_pressed(player_controller.keycode_fire) {
//             fire_projectile_event_writer.send(FireProjectileEvent::new(transform))
//         }
//     }
// }
