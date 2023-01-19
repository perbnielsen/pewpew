use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct ExplosionBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    explosion: Explosion,
    collider: Collider,
}

impl ExplosionBundle {
    pub fn new(translation: Vec3, entity: Entity) -> Self {
        Self {
            transform: Transform {
                translation,
                ..Default::default()
            },
            global_transform: GlobalTransform::default(),
            explosion: Explosion { entity },
            collider: Collider::ball(0.0),
        }
    }
}

pub fn spawn_explosion(commands: &mut Commands, translation: Vec3) {
    let mut explosion = commands.spawn_empty();
    explosion.insert(ExplosionBundle::new(translation, explosion.id()));
}

#[derive(Component)]
pub struct Explosion {
    pub entity: Entity,
}

pub fn explosion_system(
    mut commands: Commands,
    mut colliders: Query<(&mut Collider, &Explosion)>,
    time: Res<Time>,
) {
    const EXPLOSION_DURATION: f32 = 0.2;
    const EXPLOSION_SIZE: f32 = 5.0;

    for (mut collider, explosion) in colliders.iter_mut() {
        let radius = collider.as_ball_mut().unwrap().radius();

        if radius > EXPLOSION_SIZE {
            commands.entity(explosion.entity).despawn_recursive();
        } else {
            collider
                .as_ball_mut()
                .unwrap()
                .set_radius(radius + time.delta_seconds() * EXPLOSION_SIZE / EXPLOSION_DURATION);
        }
    }
}
