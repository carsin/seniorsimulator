use bevy::{prelude::*, window::PrimaryWindow};

use super::player;
use super::constants::*;

#[derive(Component)]
pub struct GunController {
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

pub fn gun_controls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<player::MainCamera>>,
    mut query: Query<(&mut GunController, &Transform), With<player::Player>>,
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().expect("Primary window not found");
    let (camera, camera_transform) = camera_query.get_single().expect("Main camera not found");

    for (mut gun_controller, player_transform) in query.iter_mut() {
        gun_controller.shoot_timer -= time.delta_seconds(); // decrement the timer
        if let Some(cursor_position) = window.cursor_position() {
            // convert cursor position from screen space to world space
            if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position).map(|ray| ray.origin.truncate()) {
                let diff = world_position - Vec2::new(player_transform.translation.x, player_transform.translation.y);
                let angle = diff.y.atan2(diff.x);

                if mouse_button_input.just_pressed(MouseButton::Left) && gun_controller.shoot_timer <= 0.0 {
                    gun_controller.shoot_timer = gun_controller.shoot_cooldown; // reset the firing cooldown timer
                    let mut spawn_transform = Transform::from_translation(player_transform.translation);
                    spawn_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

                    commands.spawn(SpriteBundle {
                        transform: spawn_transform,
                        sprite: Sprite {
                            color: Color::rgb(0.8, 0.8, 0.0),
                            custom_size: Some(Vec2::new(20.0, 3.0)),
                            ..default()
                        },
                        ..default()
                    }).insert(Bullet {
                        lifetime: BULLET_LIFETIME,
                        speed: BULLET_SPEED,
                        direction: diff.normalize(),
                    });
                }
            }
        }
    }
}
