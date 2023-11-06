use bevy::prelude::*;
use bevy_xpbd_3d::{math::Vector, prelude::*};

#[derive(Bundle)]
pub struct ExplosionBundle {
    transform: Transform,
    name: Name,
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
            name: Name::new("Explosion"),
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
        let radius = collider.shape().as_ball().unwrap().radius;

        if radius > EXPLOSION_SIZE {
            commands.entity(explosion.entity).despawn_recursive();
        } else {
            let new_radius = radius + time.delta_seconds() * EXPLOSION_SIZE / EXPLOSION_DURATION;
            collider.set_scale(Vector::ONE * new_radius, 0);
        }
    }
}
