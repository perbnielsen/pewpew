use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_system(fps_system)
        .add_system(exit_on_esc_system)
        .add_system(player_control_system)
        .add_system(movement_update_system)
        .add_startup_system(setup)
        .run();
}

// fn fps_system(time: Res<Time>) {
//     println!("Fps: {}", 1.0f32 / time.delta_seconds());
// }

fn player_control_system(
    mut players: Query<(&mut Moving, &PlayerControllerConfiguration)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut movement, player_controller) in players.iter_mut() {
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
    }
}

#[derive(Component)]
struct PlayerControllerConfiguration {
    pub keycode_left: KeyCode,
    pub keycode_right: KeyCode,
    pub keycode_forward: KeyCode,
    pub keycode_reverse: KeyCode,
}

impl PlayerControllerConfiguration {
    fn new(
        keycode_left: KeyCode,
        keycode_right: KeyCode,
        keycode_forward: KeyCode,
        keycode_reverse: KeyCode,
    ) -> Self {
        Self {
            keycode_left,
            keycode_right,
            keycode_forward,
            keycode_reverse,
        }
    }
}

#[derive(Component, Default)]
struct Moving {
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

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // Span a player
    let tank_body = assets_server.load("tank/body.gltf#Scene0");
    let tank_turrent = assets_server.load("tank/turret.gltf#Scene0");
    commands
        .spawn_bundle((
            Transform::identity(),
            GlobalTransform::identity(),
            PlayerControllerConfiguration::new(KeyCode::A, KeyCode::S, KeyCode::W, KeyCode::R),
            Moving::new(5.0, 3.0),
        ))
        .with_children(|parent| {
            parent.spawn_scene(tank_body);
            parent
                .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
                .with_children(|parent| {
                    parent.spawn_scene(tank_turrent);
                });
        });
}

fn movement_update_system(mut moving_objects: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in moving_objects.iter_mut() {
        let forward_velocity = transform.forward() * movement.velocity * time.delta_seconds();
        transform.translation += forward_velocity;
        transform.rotation *= Quat::from_rotation_y(movement.delta_yaw * time.delta_seconds());
    }
}
