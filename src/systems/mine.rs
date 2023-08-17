use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const MINE_RADIUS: f32 = 1.0;
const MINE_ARMING_DELAY: Duration = Duration::from_secs(3);
const MINE_DEPLETION_DELAY: Duration = Duration::from_secs(5);

#[derive(PartialEq, Component)]
pub enum Mine {
    Arming(Duration),
    Armed(Duration),
}

#[derive(Event)]
pub struct LayMineEvent {
    source: Entity,
}

impl LayMineEvent {
    pub(crate) fn new(source: Entity) -> Self {
        Self { source }
    }
}

pub fn mine_laying_system(
    mut commands: Commands,
    mut lay_mine_event_reader: EventReader<LayMineEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    transforms: Query<&Transform>,
    time: Res<Time>,
) {
    for event in lay_mine_event_reader.iter() {
        let mut mine = commands.spawn_empty();
        let Ok(&transform) = transforms.get(event.source) else {
            continue;
        };

        mine.insert(Mine::Arming(time.elapsed() + MINE_ARMING_DELAY));
        mine.insert(Collider::ball(MINE_RADIUS));
        mine.insert(PbrBundle {
            transform,
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

pub fn mine_lifetime_system(
    mut commands: Commands,
    mut mines: Query<(Entity, &mut Mine)>,
    time: Res<Time>,
) {
    for (entity, mine) in mines.iter_mut() {
        match *mine {
            Mine::Arming(arming_time) => {
                if time.elapsed() > arming_time {
                    commands
                        .entity(entity)
                        .remove::<Mine>()
                        .insert(Mine::Armed(time.elapsed() + MINE_DEPLETION_DELAY));

                    println!("Mine armed");
                }
            }
            Mine::Armed(depletion_time) => {
                if time.elapsed() > depletion_time {
                    commands.entity(entity).despawn();
                    println!("Boom");
                }
            }
        }
    }
}
