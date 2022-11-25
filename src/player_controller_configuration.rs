use bevy::prelude::{Component, KeyCode};
use bevy_inspector_egui::{egui, Inspectable};

#[derive(Component)]
pub struct PlayerControllerConfiguration {
    pub keycode_left: KeyCode,
    pub keycode_right: KeyCode,
    pub keycode_forward: KeyCode,
    pub keycode_reverse: KeyCode,
    pub keycode_fire: KeyCode,
}

impl PlayerControllerConfiguration {
    pub fn new(
        keycode_left: KeyCode,
        keycode_right: KeyCode,
        keycode_forward: KeyCode,
        keycode_reverse: KeyCode,
        keycode_fire: KeyCode,
    ) -> Self {
        Self {
            keycode_left,
            keycode_right,
            keycode_forward,
            keycode_reverse,
            keycode_fire,
        }
    }
}

impl Inspectable for PlayerControllerConfiguration {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        _options: Self::Attributes,
        _context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        egui::Grid::new("pos_size").show(ui, |ui| {
            ui.label("Forward");
            ui.label(format!("{:?}", self.keycode_forward));
            ui.end_row();

            ui.label("Backwards:");
            ui.label(format!("{:?}", self.keycode_reverse));
            ui.end_row();

            ui.label("Right:");
            ui.label(format!("{:?}", self.keycode_right));
            ui.end_row();

            ui.label("Left:");
            ui.label(format!("{:?}", self.keycode_left));
            ui.end_row();

            ui.label("Fire:");
            ui.label(format!("{:?}", self.keycode_fire));
        });

        false // We do not modify the data
    }
}
