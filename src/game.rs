use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::*;
use super::gun;
use super::npc;

pub fn bullet_collision_system(
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
                    npc.health -= 100.0; // adjust the damage as needed
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

pub fn draw_grid(commands: &mut Commands) {
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
