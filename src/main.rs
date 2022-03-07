use bevy::{
    core::Time,
    input::{system::exit_on_esc_system, Input},
    prelude::{App, KeyCode, Res},
    DefaultPlugins,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(fps_system)
        .add_system(exit_on_esc_system)
        .add_system(input_test)
        .run();
}

fn fps_system(time: Res<Time>) {
    println!("Fps: {}", 1.0f32 / time.delta_seconds());
}

fn input_test(keyboard_input: Res<Input<KeyCode>>) {
    // println!("Keyboard input");
    if keyboard_input.pressed(KeyCode::A) {
        println!("A was pressed");
    }
}
