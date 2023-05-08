mod systems;
use systems::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    window::close_on_esc,
};
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
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
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

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &assets_server);
    spawn_floor(&mut commands, meshes, materials);
    add_sun_light(&mut commands);
}

fn add_sun_light(commands: &mut Commands) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(50.0, 50.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn spawn_floor(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });
}

fn spawn_player(commands: &mut Commands, assets_server: &Res<AssetServer>) {
    let tank_body = assets_server.load("FancyTank/body.gltf#Scene0");
    let tank_turret = assets_server.load("FancyTank/turret.gltf#Scene0");

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
