use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::constants::*;

#[derive(Component)]
pub struct Npc {
    pub velocity: Vec2,
    pub speed: f32,
    pub health: f32,
}

pub fn spawn_npc(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.3, 0.7),
                custom_size: Some(Vec2::new(NPC_SIZE, NPC_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(NPC_SIZE / 2., NPC_SIZE / 2.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Ccd::enabled())
        .insert(TransformBundle::from(Transform::from_xyz(
            rng.gen_range(-GRID_SIZE / 2. + NPC_SIZE..GRID_SIZE / 2. - NPC_SIZE),
            rng.gen_range(-GRID_SIZE / 2. + NPC_SIZE..GRID_SIZE / 2. - NPC_SIZE),
            100.,
        )))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Npc {
            velocity: Vec2::ZERO,
            speed: NPC_SPEED,
            health: NPC_INITIAL_HEALTH,
        });
}

pub fn npc_movement_system(mut query: Query<(&mut Velocity, &mut Npc, &Transform)>) {
    for (mut velocity, mut npc, transform) in query.iter_mut() {
        // Randomly decide whether to change direction
        if rand::random::<f32>() < 0.01 {
            npc.velocity = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                .normalize_or_zero()
                * npc.speed;
        }

        // Set the linear velocity
        velocity.linvel = npc.velocity;

        let clamped_x = transform.translation.x.clamp(
            -GRID_SIZE / 2.0 + NPC_SIZE / 2.0,
            GRID_SIZE / 2.0 - NPC_SIZE / 2.0,
        );
        let clamped_y = transform.translation.y.clamp(
            -GRID_SIZE / 2.0 + NPC_SIZE / 2.0,
            GRID_SIZE / 2.0 - NPC_SIZE / 2.0,
        );

        // Apply clamping
        velocity.linvel.x = if clamped_x != transform.translation.x {
            0.0
        } else {
            velocity.linvel.x
        };
        velocity.linvel.y = if clamped_y != transform.translation.y {
            0.0
        } else {
            velocity.linvel.y
        };
    }
}
