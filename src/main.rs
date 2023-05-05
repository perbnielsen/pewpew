mod systems;
use systems::*;

use bevy::{prelude::*, window::close_on_esc};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

// [ ] Mines
//   [*] Lay mines using event
//   [*] Mines activate after fixed period
//   [*] Mines explode after fixed period
//   [ ] Mines explode on proximity to vehicle
// [ ] AI
// [ ] UI
// [ ] Level loading
// [ ] Death
// [ ] Collision detection

fn main() {
    App::new()
        .add_event::<LayMineEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .register_inspectable::<Moving>()
        // .register_inspectable::<PlayerControllerConfiguration>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        // .register_inspectable::<Moving>()
        // .register_inspectable::<PlayerControllerConfiguration>()
        .add_system(close_on_esc)
        .add_system(player_control_system)
        .add_system(movement_update_system)
        .add_system(auto_despawn_system)
        .add_system(explosion_system)
        .add_system(mine_laying_system)
        .add_system(mine_lifetime_system)
        .run();
}

#[derive(Component, Default)]
pub struct Projectile {}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &assets_server);
    spawn_floor(&mut commands);
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(30.0, 30.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn spawn_floor(commands: &mut Commands) {
    commands.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}

fn spawn_player(commands: &mut Commands, assets_server: &Res<AssetServer>) {
    let tank_body = assets_server.load("tank/body.gltf#Scene0");
    let tank_turret = assets_server.load("tank/turret.gltf#Scene0");

    commands
        .spawn((
            Name::new("Tank"),
            SceneBundle {
                scene: tank_body,
                ..Default::default()
            },
            PlayerControllerConfiguration::new(
                KeyCode::A,
                KeyCode::S,
                KeyCode::W,
                KeyCode::R,
                KeyCode::Space,
                KeyCode::M,
            ),
            Moving::new(10.0, 3.0),
            RigidBody::KinematicVelocityBased,
            Velocity::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Turret"),
                SceneBundle {
                    scene: tank_turret,
                    ..Default::default()
                },
            ));
        });
}
