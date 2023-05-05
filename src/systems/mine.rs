use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct MineBundle {
    name: Name,
    mine: Mine,
    collider: Collider,
    mesh: PbrBundle,
}

const MINE_ARMING_DELAY: Duration = Duration::from_secs(3);
const MINE_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct Mine {
    creation: Duration,
    state: MineState,
}

#[derive(PartialEq)]
enum MineState {
    Arming,
    Armed,
}

pub struct LayMineEvent {
    transform: Transform,
}

impl LayMineEvent {
    pub(crate) fn new(transform: &Transform) -> Self {
        Self {
            transform: transform.clone(),
        }
    }
}

impl Mine {
    fn new(time: Time) -> Self {
        Self {
            creation: time.elapsed(),
            state: MineState::Arming,
        }
    }

    // pub fn create_mine(translation: Vec3, time: Duration) -> MineBundle {
    //     MineBundle {
    //         name: Name::from("Mine"),
    //         collider: Collider::ball(MINE_RADIUS),
    //         mine: Mine {
    //             creation: time,
    //             state: MineState::Arming,
    //         },
    //         mesh: PbrBundle {
    //             transform: Transform {
    //                 translation,
    //                 ..Default::default()
    //             },
    //             mesh: meshes.add(Mesh::from(shape::Icosphere {
    //                 radius: MINE_RADIUS,
    //                 subdivisions: 3,
    //             })),
    //             ..Default::default()
    //         },
    //     }
    // }
}

pub fn mine_laying_system(
    mut commands: Commands,
    mut lay_mine_event_reader: EventReader<LayMineEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in lay_mine_event_reader.iter() {
        let mut mine = commands.spawn_empty();

        const MINE_RADIUS: f32 = 1.0;
        mine.insert(Collider::ball(MINE_RADIUS));
        mine.insert(PbrBundle {
            transform: event.transform,
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: MINE_RADIUS,
                    subdivisions: 3,
                })
                .unwrap(),
            ),
            ..Default::default()
        });
    }
}

pub fn mine_system(mut commands: Commands, mut mines: Query<&mut Mine>, time: Res<Time>) {
    for mut mine in mines.iter_mut() {
        if mine.state == MineState::Arming && time.elapsed() > mine.creation + MINE_ARMING_DELAY {
            mine.state = MineState::Armed;
            println!("Mine armed");
        }
    }
}
