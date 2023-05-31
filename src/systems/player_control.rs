use bevy::prelude::*;
// use bevy_inspector_egui::egui;

use super::{mine::LayMineEvent, movement_update::Moving, FireProjectileEvent};

pub fn player_control_system(
    mut players: Query<(
        Entity,
        &mut Moving,
        &PlayerControllerConfiguration,
        &Transform,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut lay_mine_event_writer: EventWriter<LayMineEvent>,
    mut fire_projectile_event_writer: EventWriter<FireProjectileEvent>,
) {
    for (entity, mut movement, player_controller, transform) in players.iter_mut() {
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
            fire_projectile_event_writer.send(FireProjectileEvent::new(entity))
        }
        if keyboard_input.just_pressed(player_controller.keycode_lay_mine) {
            lay_mine_event_writer.send(LayMineEvent::new(transform));
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
