use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {}

#[derive(Component)]
pub struct Tank {
    pub entity: Entity,
}
pub fn aim_turret(
    tanks: Query<&Transform, Without<Turret>>,
    mut turrets: Query<(&mut Transform, &Parent), With<Turret>>,
) {
    for (mut turret_transform, parent) in &mut turrets {
        if let Ok(tank_transform) = tanks.get(parent.get()) {
            let target = tank_transform
                .compute_matrix()
                .inverse()
                .transform_point3(Vec3::X);
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
