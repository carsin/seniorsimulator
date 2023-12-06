use bevy::prelude::*;
use constants::*;

mod input;
mod gun;
mod player;
mod constants;
mod npc;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, player::player_movement_sys, player::camera_follow_player_system, bullet_movement_system, gun::gun_controls, npc::npc_movement_system))
        .run();
}

fn setup(mut commands: Commands) {
    // setup camera
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle).insert(player::MainCamera);

    draw_grid(&mut commands);

    // spawn gun as a child of the player
    let gun_entity = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(15.0, 5.0)), 
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(2., 0., 200.)),
            ..Default::default()
        })
        .insert(gun::Gun {
            shoot_cooldown: 0.2,
            shoot_timer: 0.0,
        })
        .id();
    // spawn player box
    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.5),
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
            ..Default::default()
        })
        .insert(player::Player {
            velocity: Vec3::ZERO,
            max_velocity: PLAYER_MAX_SPEED,
            acceleration: PLAYER_ACCEL,
        })
        .add_child(gun_entity);
    
    // Spawn an NPC
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.3, 0.7), // Different color to distinguish from the player
            custom_size: Some(Vec2::new(30.0, 30.0)), // Customize as needed
            ..Default::default()
        },
        transform: Transform::from_xyz(100.0, 100.0, 0.0), // Initial position
        ..Default::default()
    })
    .insert(npc::Npc {
        velocity: Vec3::new(0.0, 0.0, 0.0),
        speed: NPC_SPEED,
    });
}


fn bullet_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &gun::Bullet, &mut Transform)>,
) {
    for (entity, bullet, mut transform) in query.iter_mut() {
        let movement = bullet.direction * bullet.speed * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // Decrease lifetime and despawn if lifetime is over
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Update the lifetime of the bullet
            commands.entity(entity).insert(gun::Bullet {
                lifetime: bullet.lifetime - time.delta_seconds(),
                ..*bullet
            });
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
