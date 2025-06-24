mod systems;

use std::f32::consts::PI;

use systems::*;

use avian3d::prelude::*;
use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Component)]
struct RedTank;

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
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
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
        .add_systems(Update, (print_collisions, apply_red_tank_color))
        .run();
}

fn load_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &game_assets);
    spawn_second_player(&mut commands, &game_assets);
    spawn_floor(&mut commands, meshes, materials);
    add_sun_light(&mut commands);
}

fn add_sun_light(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 50.0,
            maximum_distance: 200.0,
            first_cascade_far_bound: 75.0,
            overlap_proportion: 0.2,
        }
        .build(),
    ));
}

fn print_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for collision in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            collision.0, collision.1,
        );
    }
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::new(10.0, 0.0, 10.0), Vec3::Y),
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    MouseCollisionLayer,
}

fn spawn_floor(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _spawn = commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        CollisionLayers::new(
            [GameLayer::MouseCollisionLayer],
            [GameLayer::MouseCollisionLayer; 0],
        ),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::default(),
    ));
}

fn spawn_player(commands: &mut Commands, game_assets: &Res<GameAssets>) {
    let tank = commands
        .spawn((
            Name::new("Tank"),
            Tank {},
            SceneRoot(game_assets.get_asset(GameAssetName::TankBody)),
            Transform::default(),
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
                Transform::from_xyz(0.0, 3.0, 0.0),
                Collider::capsule(2.0, 4.0),
            ));
        })
        .id();

    let turret = commands
        .spawn((
            Name::new("Turret"),
            Turret { tank },
            SceneRoot(game_assets.get_asset(GameAssetName::TankTurret)),
            Transform::default(),
        ))
        .id();

    commands.entity(turret).insert(ChildOf(tank));
}

fn spawn_second_player(commands: &mut Commands, game_assets: &Res<GameAssets>) {
    let tank = commands
        .spawn((
            Name::new("Tank 2"),
            Tank {},
            RedTank,
            SceneRoot(game_assets.get_asset(GameAssetName::TankBody)),
            Transform::from_xyz(20.0, 0.0, 20.0), // Different starting position
            Moving::new(10.0, 3.0),
            RigidBody::Kinematic,
            LinearVelocity::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(0.0, 3.0, 0.0),
                Collider::capsule(2.0, 4.0),
            ));
        })
        .id();

    let turret = commands
        .spawn((
            Name::new("Turret 2"),
            Turret { tank },
            RedTank,
            SceneRoot(game_assets.get_asset(GameAssetName::TankTurret)),
            Transform::default(),
        ))
        .id();

    commands.entity(turret).insert(ChildOf(tank));
}

fn apply_red_tank_color(
    red_tank_roots: Query<Entity, With<RedTank>>,
    children: Query<&Children>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
    mut commands: Commands,
) {
    for red_tank_root in &red_tank_roots {
        // Apply red color to the root entity if it has a material
        if material_handles.get(red_tank_root).is_ok() {
            let red_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            });
            commands
                .entity(red_tank_root)
                .insert(MeshMaterial3d(red_material));
        }

        // Recursively apply red color to all children
        apply_red_to_children(
            red_tank_root,
            &children,
            &mut materials,
            &material_handles,
            &mut commands,
        );
    }
}

fn apply_red_to_children(
    entity: Entity,
    children: &Query<&Children>,
    materials: &mut Assets<StandardMaterial>,
    material_handles: &Query<&MeshMaterial3d<StandardMaterial>>,
    commands: &mut Commands,
) {
    if let Ok(children_entities) = children.get(entity) {
        for child in children_entities.iter() {
            // Apply red color to this child if it has a material
            if material_handles.get(child).is_ok() {
                let red_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.2, 0.2),
                    ..default()
                });
                commands.entity(child).insert(MeshMaterial3d(red_material));
            }

            // Recursively apply to grandchildren
            apply_red_to_children(child, children, materials, material_handles, commands);
        }
    }
}
