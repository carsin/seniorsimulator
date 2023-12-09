use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::*;
use super::gun;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub max_velocity: f32,
    pub acceleration: f32,
}

pub fn spawn_player(commands: &mut Commands) {
    // spawn gun as a child of the player
    let gun_entity = commands
        .spawn(SpriteBundle {
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
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.5),
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 100.0)))
        .insert(Damping { linear_damping: FRICTION, angular_damping: FRICTION })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0
        })
        .insert(Player {
            velocity: Vec3::ZERO,
            max_velocity: PLAYER_MAX_SPEED,
            acceleration: PLAYER_ACCEL,
        })
        .add_child(gun_entity);

}

pub fn player_movement_sys(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player, &mut Transform)>,
) {
    for (mut velocity, player, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        // Apply acceleration
        let new_velocity = direction * player.acceleration * time.delta_seconds();
        velocity.linvel = (velocity.linvel + new_velocity).clamp_length_max(player.max_velocity);

        // Manually clamp the player's position within the grid
        transform.translation.x = transform.translation.x.clamp(
            -GRID_SIZE / 2.0 + PLAYER_SIZE / 2.0,
            GRID_SIZE / 2.0 - PLAYER_SIZE / 2.0,
        );
        transform.translation.y = transform.translation.y.clamp(
            -GRID_SIZE / 2.0 + PLAYER_SIZE / 2.0,
            GRID_SIZE / 2.0 - PLAYER_SIZE / 2.0,
        );
    }
}

pub fn camera_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    // Calculate camera position, clamping to grid boundaries
    let camera_x = player_transform
        .translation
        .x
        .clamp(-GRID_SIZE / 2., GRID_SIZE / 2.);
    let camera_y = player_transform
        .translation
        .y
        .clamp(-GRID_SIZE / 2., GRID_SIZE / 2.);

    camera_transform.translation.x = camera_x;
    camera_transform.translation.y = camera_y;
}
