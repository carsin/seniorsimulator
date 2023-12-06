use bevy::prelude::*;
use super::constants::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub max_velocity: f32,
    pub acceleration: f32,
}

pub fn player_movement_sys(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let mut direction = Vec3::ZERO;
    for (mut transform, mut player) in query.iter_mut() {

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

         // Calculate new velocity
        let new_velocity = (player.velocity + (direction * player.acceleration) * time.delta_seconds())
            .clamp_length_max(player.max_velocity) * FRICTION;

        // Update player's velocity
        player.velocity = new_velocity;

        // Update position and clamp within grid
        transform.translation += player.velocity;

        // Update position and clamp within grid
        transform.translation += player.velocity;
        transform.translation.x = transform.translation.x.clamp(-GRID_SIZE / 2. + PLAYER_SIZE / 2., GRID_SIZE / 2. - PLAYER_SIZE / 2.);
        transform.translation.y = transform.translation.y.clamp(-GRID_SIZE / 2. + PLAYER_SIZE / 2., GRID_SIZE / 2. - PLAYER_SIZE / 2.);
    }
}

pub fn camera_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    // Calculate camera position, clamping to grid boundaries
    let camera_x = player_transform.translation.x.clamp(-GRID_SIZE / 2., GRID_SIZE / 2.);
    let camera_y = player_transform.translation.y.clamp(-GRID_SIZE / 2., GRID_SIZE / 2.);

    camera_transform.translation.x = camera_x;
    camera_transform.translation.y = camera_y;
}
