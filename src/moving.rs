use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Default, Inspectable)]
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
            ..Default::default()
        }
    }
}
