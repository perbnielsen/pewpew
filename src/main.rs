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
        if keyboard_input.pressed(player_controller.keycode_left) {
            println!("Turn left");
            movement.delta_yaw -= 1.0;
        }
        if keyboard_input.pressed(player_controller.keycode_right) {
            println!("Turn right");
            movement.delta_yaw += 1.0;
        }
        if keyboard_input.pressed(player_controller.keycode_forward) {
            println!("Turn forward");
            movement.velocity += 1.0;
        }
        if keyboard_input.pressed(player_controller.keycode_reverse) {
            println!("Turn reverse");
            movement.velocity -= 1.0;
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
    pub velocity: f32,
    pub delta_yaw: f32,
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    assets_server: Res<AssetServer>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
    // Span a player
    commands.spawn_bundle((
        Transform::identity(),
        GlobalTransform::identity(),
        PlayerControllerConfiguration::new(KeyCode::A, KeyCode::S, KeyCode::W, KeyCode::R),
        Moving::default(),
    ));

    // let transform = Transform::from_xyz(0.0, 0.0, 0.0);
    let mesh = assets_server.load("tank/turret.gltf#Scene0");
    commands.spawn_scene(mesh);
    let mesh = assets_server.load("tank/body.gltf#Scene0");
    commands.spawn_scene(mesh);
    // commands.spawn_bundle(PbrBundle {
    // mesh: meshes.add(Mesh::from(shape::Box::new(5.27, 2.79, 1.02))),
    // mesh, //assets_server.load("tank/turret.gltf#Scene0"),
    // transform,
    // material: materials.add(StandardMaterial {
    //     base_color: Color::INDIGO,
    //     perceptual_roughness: 1.0,
    //     ..Default::default()
    // }),
    // ..Default::default()
    // });
}

fn movement_update_system(mut moving_objects: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in moving_objects.iter_mut() {
        // TODO: rotate transform according to movement
        let forward_velocity = transform.forward() * movement.velocity * time.delta_seconds();
        transform.translation += forward_velocity;
    }
}
