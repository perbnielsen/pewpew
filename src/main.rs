mod systems;

use std::f32::consts::PI;

use systems::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    window::close_on_esc,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .init_resource::<GameAssets>()
        .add_state::<AppState>()
        .add_event::<LayMineEvent>()
        .add_event::<FireProjectileEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(load_game_assets.in_schedule(OnEnter(AppState::Loading)))
        .add_system(loading_assets.in_set(OnUpdate(AppState::Loading)))
        .add_system(load_level.in_schedule(OnEnter(AppState::Game)))
        .add_systems(
            (
                close_on_esc,
                player_control_system,
                movement_update_system,
                auto_despawn_system,
                explosion_system,
                fire_projectile,
                mine_laying_system,
                mine_lifetime_system,
                aim_turret,
            )
                .in_set(OnUpdate(AppState::Game)),
        )
        .run();
}

fn load_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, game_assets);
    spawn_floor(&mut commands, meshes, materials);
    add_sun_light(&mut commands);
}

fn add_sun_light(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 50.0,
            maximum_distance: 200.0,
            first_cascade_far_bound: 75.0,
            overlap_proportion: 0.2,
        }
        .build(),
        ..default()
    });
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(75.0, 75.0, 0.0)
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

fn spawn_player(commands: &mut Commands, game_assets: Res<GameAssets>) {
    let tank = commands
        .spawn((
            Name::new("Tank"),
            SceneBundle {
                scene: game_assets.get_asset(GameAssetName::TankBody),
                ..Default::default()
            },
            PlayerControllerConfiguration::new(
                KeyCode::A,
                KeyCode::D,
                KeyCode::W,
                KeyCode::S,
                KeyCode::Space,
                KeyCode::M,
            ),
            Moving::new(10.0, 3.0),
            RigidBody::KinematicVelocityBased,
            Velocity::default(),
        ))
        .id();

    let turret = commands
        .spawn((
            Name::new("Turret"),
            Turret { tank },
            SceneBundle {
                scene: game_assets.get_asset(GameAssetName::TankTurret),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(turret).set_parent(tank);
}
