use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::constants::*;

#[derive(Component)]
pub struct Npc {
    pub velocity: Vec2,
    pub target_velocity: Vec2,
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
            rng.gen_range(-GRID_SIZE / 2. + NPC_SIZE * 2.0..GRID_SIZE / 2. - NPC_SIZE * 2.),
            rng.gen_range(-GRID_SIZE / 2. + NPC_SIZE * 2.0..GRID_SIZE / 2. - NPC_SIZE * 2.),
            100.,
        )))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Npc {
            velocity: Vec2::ZERO,
            target_velocity: Vec2::ZERO,
            speed: NPC_SPEED,
            health: NPC_INITIAL_HEALTH,
        });
}

pub fn npc_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Npc, &Transform)>,
) {
    let dt = time.delta_seconds();
    let mut rng = rand::thread_rng();

    for (mut velocity, mut npc, transform) in query.iter_mut() {
        // Randomly decide whether to change direction or stop
        if rng.gen::<f32>() < 0.01 {
            let stop_moving = rng.gen_bool(0.30); // chance to stop moving
            npc.target_velocity = if stop_moving {
                Vec2::ZERO
            } else {
                Vec2::new(rng.gen_range(-1.5..1.5), rng.gen_range(-1.5..1.5))
                    .normalize_or_zero() * npc.speed
            };
        }

        // Apply acceleration
        npc.velocity = npc.velocity.lerp(npc.target_velocity, 5.0 * dt);
        velocity.linvel = npc.velocity;

        // Clamping logic to keep NPCs within the grid
        let clamped_x = transform.translation.x.clamp(
            -GRID_SIZE / 2.0 + NPC_SIZE / 2.0,
            GRID_SIZE / 2.0 - NPC_SIZE / 2.0,
        );
        let clamped_y = transform.translation.y.clamp(
            -GRID_SIZE / 2.0 + NPC_SIZE / 2.0,
            GRID_SIZE / 2.0 - NPC_SIZE / 2.0,
        );

        // Apply clamping
        if clamped_x != transform.translation.x {
            velocity.linvel.x = 0.0;
        }
        if clamped_y != transform.translation.y {
            velocity.linvel.y = 0.0;
        }
    }
}
