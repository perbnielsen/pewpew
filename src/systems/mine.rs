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

pub fn mine_system(mut commands: Commands, mut mines: Query<(&mut Mine)>, time: Res<Time>) {
    for mut mine in mines.iter_mut() {
        if mine.state == MineState::Arming && time.elapsed() > mine.creation + MINE_ARMING_DELAY {
            mine.state = MineState::Armed;
            println!("Mine armed");
        }
    }
}
