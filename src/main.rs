use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_system(fps_system)
        .add_system(exit_on_esc_system)
        .add_system(player_control_system)
        .add_system(movement_update_system)
        .add_system(auto_despawn_system)
        .add_startup_system(setup)
        // .add_system(print_children_names)
        .run();
}

// fn fps_system(time: Res<Time>) {
//     println!("Fps: {}", 1.0f32 / time.delta_seconds());
// }

// fn print_children_names(
//     transforms: Query<(&Transform, &Children, &Name)>,
//     transform_query: Query<(&Transform, &Name)>,
// ) {
//     for (_, children, name) in transforms.iter() {
//         println!("{}:", name.as_str());
//         for child in children.iter() {
//             if let Ok((_, name)) = transform_query.get(*child) {
//                 println!("    {}", name.as_str());
//             } else {
//                 println!("    Unnamed child");
//             }
//         }
//     }
// }

fn player_control_system(
    mut players: Query<(
        &mut Moving,
        &PlayerControllerConfiguration,
        &Transform,
        &Children,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
            let transform = transform.with_translation(*transform * Vec3::new(0.0, 1.52, -3.51));
            let mut bullet = commands.spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 3,
                })),
                ..Default::default()
            });
            bullet.insert(Projectile::default()).insert(Moving {
                velocity: 40.0,
                ..Default::default()
            });
            let id = bullet.id();
            let auto_despawner = AutoDespawn {
                entity: id,
                time_to_live: 1.0,
            };
            bullet.insert(auto_despawner);
        }
    }
}

#[derive(Component)]
struct PlayerControllerConfiguration {
    pub keycode_left: KeyCode,
    pub keycode_right: KeyCode,
    pub keycode_forward: KeyCode,
    pub keycode_reverse: KeyCode,
    pub keycode_fire: KeyCode,
}

impl PlayerControllerConfiguration {
    fn new(
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

#[derive(Debug, Component)]
struct AutoDespawn {
    pub entity: Entity,
    pub time_to_live: f32,
}

fn auto_despawn_system(
    mut commands: Commands,
    mut auto_despawners: Query<&mut AutoDespawn>,
    time: Res<Time>,
) {
    for mut auto_despawner in auto_despawners.iter_mut() {
        auto_despawner.time_to_live -= time.delta_seconds();
        if auto_despawner.time_to_live < 0.0 {
            commands.entity(auto_despawner.entity).despawn_recursive();
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

#[derive(Component, Default)]
struct Projectile {}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(30.0, 30.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    span_player(&mut commands, &assets_server);
}

fn span_player(commands: &mut Commands, assets_server: &Res<AssetServer>) {
    let tank_body = assets_server.load("tank/body.gltf#Scene0");
    let tank_turret = assets_server.load("tank/turret.gltf#Scene0");

    commands
        .spawn_bundle((
            Transform::identity(),
            GlobalTransform::identity(),
            PlayerControllerConfiguration::new(
                KeyCode::A,
                KeyCode::S,
                KeyCode::W,
                KeyCode::R,
                KeyCode::Space,
            ),
            Moving::new(10.0, 3.0),
        ))
        .with_children(|parent| {
            parent.spawn_scene(tank_body);
            parent
                .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
                .with_children(|parent| {
                    parent.spawn_scene(tank_turret);
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
