mod systems;

use std::f32::consts::PI;

use systems::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::*;

// [ ] Mines
//     [*] Lay mines using event
//     [*] Mines activate after fixed period
//     [*] Mines explode after fixed period
//     [ ] Mines explode when shot
//     [ ] Mines explode on proximity to vehicle
// [ ] AI
// [ ] UI
// [ ] Level creation
//     [ ] Level loading
//     [ ] Level saving
//     [ ] Level editing
// [ ] Death
// [ ] Collision detection
// [ ] Get rid of auto_despawn

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum AppState {
    #[default]
    Loading,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .init_state::<AppState>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .init_resource::<GameAssets>()
        .add_event::<LayMineEvent>()
        .add_event::<FireProjectileEvent>()
        .add_systems(OnEnter(AppState::Loading), load_game_assets)
        .add_systems(Update, loading_assets.run_if(in_state(AppState::Loading)))
        .add_systems(OnEnter(AppState::Game), load_level)
        .add_systems(
            Update,
            (
                player_control_system,
                movement_update_system,
                auto_despawn_system,
                explosion_system,
                fire_projectile,
                mine_laying_system,
                mine_lifetime_system,
                aim_turret,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, print_collisions)
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
            illuminance: 10000.0,
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

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(75.0, 75.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    MouseCollisionLayer,
}

fn spawn_floor(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _spawn = commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        CollisionLayers::new([CollisionLayer::MouseCollisionLayer], 0),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ));
}

fn spawn_player(commands: &mut Commands, game_assets: Res<GameAssets>) {
    let tank = commands
        .spawn((
            Name::new("Tank"),
            Tank {},
            SceneBundle {
                scene: game_assets.get_asset(GameAssetName::TankBody),
                ..default()
            },
            PlayerControllerConfiguration::new(
                KeyCode::KeyA,
                KeyCode::KeyD,
                KeyCode::KeyW,
                KeyCode::KeyS,
                KeyCode::Space,
                KeyCode::KeyM,
            ),
            Moving::new(10.0, 3.0),
            RigidBody::Kinematic,
            LinearVelocity::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
                Collider::capsule(2.0, 4.0),
            ));
        })
        .id();

    let turret = commands
        .spawn((
            Name::new("Turret"),
            Turret { tank },
            SceneBundle {
                scene: game_assets.get_asset(GameAssetName::TankTurret),
                ..default()
            },
        ))
        .id();

    commands.entity(turret).set_parent(tank);
}
