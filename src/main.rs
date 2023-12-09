use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use constants::*;

mod constants;
mod gun;
mod input;
mod npc;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                player::player_movement_sys,
                player::camera_follow_player_system,
                bullet_collision_system,
                gun::gun_controls,
                npc::npc_movement_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // remove rapier's default gravity
    rapier_config.gravity = Vec2::ZERO;

    // setup camera
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle).insert(player::MainCamera);

    // draw background
    draw_grid(&mut commands);

    // spawn player
    player::spawn_player(&mut commands);

    // Spawn some NPCs
    for _ in 0..5 {
        npc::spawn_npc(&mut commands);
    }
}

// fn bullet_movement_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &gun::Bullet, &mut Transform)>,
// ) {
//     for (entity, bullet, mut transform) in query.iter_mut() {
//         let movement = bullet.direction * bullet.speed * time.delta_seconds();
//         transform.translation.x += movement.x;
//         transform.translation.y += movement.y;
//
//         // Decrease lifetime and despawn if lifetime is over
//         if bullet.lifetime <= 0.0 {
//             commands.entity(entity).despawn();
//         } else {
//             // Update the lifetime of the bullet
//             commands.entity(entity).insert(gun::Bullet {
//                 lifetime: bullet.lifetime - time.delta_seconds(),
//                 ..*bullet
//             });
//         }
//     }
// }

fn bullet_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    bullet_query: Query<Entity, With<gun::Bullet>>,
    mut npc_query: Query<&mut npc::Npc>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Check if one of the entities is a bullet
            let (bullet_entity, other_entity) = if bullet_query.get(*entity1).is_ok() {
                (Some(*entity1), *entity2)
            } else if bullet_query.get(*entity2).is_ok() {
                (Some(*entity2), *entity1)
            } else {
                (None, Entity::from_raw(0)) // Invalid entity
            };

            if let Some(bullet_entity) = bullet_entity {
                // despawn the bullet
                commands.entity(bullet_entity).despawn();

                // check if the other entity is an NPC and reduce its health
                if let Ok(mut npc) = npc_query.get_mut(other_entity) {
                    npc.health -= 20.0; // adjust the damage as needed
                    println!("ouch! new health: {}", npc.health);
                    if npc.health <= 0.0 {
                        // handle NPC death
                        commands.entity(other_entity).despawn();
                        // spawn new npc
                        npc::spawn_npc(&mut commands);
                    }
                }
            }
        }
    }
}

fn draw_grid(commands: &mut Commands) {
    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 * GRID_SIZE / MAP_SIZE as f32 - GRID_SIZE / 2.,
                0.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_SIZE, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                i as f32 * GRID_SIZE / MAP_SIZE as f32 - GRID_SIZE / 2.,
                0.,
                0.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, GRID_SIZE)),
                ..default()
            },
            ..default()
        });
    }
}
