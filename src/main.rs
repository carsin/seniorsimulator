use bevy::prelude::*;
use constants::*;

mod input;
mod gun;
mod player;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, player::player_movement_sys)
        .add_systems(Update, player::camera_follow_player_system)
        .add_systems(Update, bullet_movement_system)
        .add_systems(Update, gun::gun_controls)
        .run();
}

fn setup(mut commands: Commands) {
    // setup camera
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle).insert(player::MainCamera);

    draw_grid(&mut commands);

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
        .insert(gun::GunController {
            shoot_cooldown: 0.2,
            shoot_timer: 0.0,
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
