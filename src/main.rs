use bevy::{
    core::Time,
    input::{system::exit_on_esc_system, Input},
    math::Vec3,
    prelude::{App, Commands, Component, KeyCode, Query, Res, Transform},
    DefaultPlugins,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(fps_system)
        .add_system(exit_on_esc_system)
        .add_system(control_players_system)
        .add_system(movement_update_system)
        .add_startup_system(create_level)
        .run();
}

fn fps_system(time: Res<Time>) {
    println!("Fps: {}", 1.0f32 / time.delta_seconds());
}

fn control_players_system(
    players: Query<(&Moving, &PlayerControllerConfiguration)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (movement, playerController) in players.iter() {
        // compare playerControllersConfiguration to keys pressed to update the movements
        if keyboard_input.pressed(playerController.keycode_left) {
            println!("Turn left");
        }
        if keyboard_input.pressed(playerController.keycode_right) {
            println!("Turn right");
        }
        if keyboard_input.pressed(playerController.keycode_forward) {
            println!("Turn forward");
        }
        if keyboard_input.pressed(playerController.keycode_reverse) {
            println!("Turn reverse");
        }
    }
}

#[derive(Component)]
struct PlayerControllerConfiguration {
    pub keycode_left: KeyCode,
    pub keycode_right: KeyCode,
    pub keycode_forward: KeyCode,
    pub keycode_reverse: KeyCode,
}

impl Default for PlayerControllerConfiguration {
    fn default() -> Self {
        Self {
            keycode_left: KeyCode::A,
            keycode_right: KeyCode::S,
            keycode_forward: KeyCode::W,
            keycode_reverse: KeyCode::R,
        }
    }
}

#[derive(Component, Default)]
struct Moving {
    pub velocity: f32,
}

fn create_level(mut commands: Commands) {
    commands.spawn_bundle((
        Transform::identity(),
        PlayerControllerConfiguration::default(),
        Moving::default(),
    ));
}

fn movement_update_system(mut moving_objects: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in moving_objects.iter_mut() {
        let translation =
            transform.rotation.mul_vec3(Vec3::X) * movement.velocity * time.delta_seconds();
        transform.translation += translation;
    }
}
