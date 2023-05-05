use bevy::prelude::*;
// use bevy_inspector_egui::egui;
use bevy_rapier3d::prelude::*;

use crate::{systems::AutoDespawn, Projectile};

use super::{mine::LayMineEvent, movement_update::Moving};

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
    mut lay_mine_event_writer: EventWriter<LayMineEvent>,
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
            fire_bullet(&mut commands, transform, &mut meshes);
        }
        if keyboard_input.just_pressed(player_controller.keycode_lay_mine) {
            lay_mine_event_writer.send(LayMineEvent::new(transform));
            // lay_mine(&mut commands, transform, &mut meshes);
        }
    }
}

#[derive(Component)]
pub struct PlayerControllerConfiguration {
    pub keycode_left: KeyCode,
    pub keycode_right: KeyCode,
    pub keycode_forward: KeyCode,
    pub keycode_reverse: KeyCode,
    pub keycode_fire: KeyCode,
    pub keycode_lay_mine: KeyCode,
}

impl PlayerControllerConfiguration {
    pub fn new(
        keycode_left: KeyCode,
        keycode_right: KeyCode,
        keycode_forward: KeyCode,
        keycode_reverse: KeyCode,
        keycode_fire: KeyCode,
        keycode_lay_mine: KeyCode,
    ) -> Self {
        Self {
            keycode_left,
            keycode_right,
            keycode_forward,
            keycode_reverse,
            keycode_fire,
            keycode_lay_mine,
        }
    }
}

// impl Reflect for PlayerControllerConfiguration {
//     type Attributes = ();

//     fn ui(
//         &mut self,
//         ui: &mut bevy_inspector_egui::egui::Ui,
//         _options: Self::Attributes,
//         _context: &mut bevy_inspector_egui::Context,
//     ) -> bool {
//         egui::Grid::new("pos_size").show(ui, |ui| {
//             ui.label("Forward");
//             ui.label(format!("{:?}", self.keycode_forward));
//             ui.end_row();

//             ui.label("Backwards:");
//             ui.label(format!("{:?}", self.keycode_reverse));
//             ui.end_row();

//             ui.label("Right:");
//             ui.label(format!("{:?}", self.keycode_right));
//             ui.end_row();

//             ui.label("Left:");
//             ui.label(format!("{:?}", self.keycode_left));
//             ui.end_row();

//             ui.label("Fire:");
//             ui.label(format!("{:?}", self.keycode_fire));
//         });

//         false // We do not modify the data
//     }
// }

fn fire_bullet(commands: &mut Commands, transform: &Transform, meshes: &mut ResMut<Assets<Mesh>>) {
    const BULLET_FIRE_OFFSET: Vec3 = Vec3::new(0.0, 1.5, -3.5);
    const BULLET_RADIUS: f32 = 0.2;
    const BULLET_LIFETIME: f32 = 1.0;
    const BULLET_VELOCITY: f32 = 20.0;

    let mut bullet = commands.spawn_empty();
    let entity_id = bullet.id();
    bullet.insert((
        PbrBundle {
            transform: transform.with_translation(*transform * BULLET_FIRE_OFFSET),
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: BULLET_RADIUS,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            ..Default::default()
        },
        Projectile::default(),
        RigidBody::KinematicVelocityBased,
        Velocity::linear(transform.forward() * BULLET_VELOCITY),
        Collider::ball(BULLET_RADIUS),
        AutoDespawn {
            entity: entity_id,
            time_to_live: BULLET_LIFETIME,
        },
    ));
}

// fn lay_mine(commands: &mut Commands, transform: &Transform, meshes: &mut ResMut<Assets<Mesh>>) {
//     let mut mine = commands.spawn_empty();

//     const MINE_RADIUS: f32 = 1.0;
//     mine.insert(Collider::ball(MINE_RADIUS));
//     mine.insert(PbrBundle {
//         transform: *transform,
//         mesh: meshes.add(
//             Mesh::try_from(shape::Icosphere {
//                 radius: MINE_RADIUS,
//                 subdivisions: 3,
//             })
//             .unwrap(),
//         ),
//         ..Default::default()
//     });
// }
