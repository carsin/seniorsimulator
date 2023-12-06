use bevy::{prelude::*, window::PrimaryWindow};

use super::constants::*;
use super::player;

#[derive(Component)]
pub struct Gun {
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
    mut player_query: Query<(&Transform, &Children), (With<player::Player>, Without<Gun>)>,
    mut gun_query: Query<(&mut Gun, &mut Transform), With<Gun>>,
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().expect("Primary window not found");
    let (camera, camera_transform) = camera_query.get_single().expect("Main camera not found");

    for (player_transform, children) in player_query.iter_mut() {
        if let Some(gun_entity) = children
            .iter()
            .find(|&&entity| gun_query.get_component::<Gun>(entity).is_ok())
        {
            if let Ok((mut gun, mut gun_transform)) = gun_query.get_mut(*gun_entity) {
                gun.shoot_timer -= time.delta_seconds(); // decrement the timer

                if let Some(cursor_position) = window.cursor_position() {
                    if let Some(world_position) = camera
                        .viewport_to_world(camera_transform, cursor_position)
                        .map(|ray| ray.origin.truncate())
                    {
                        let diff = world_position
                            - Vec2::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                            );
                        let angle = diff.y.atan2(diff.x);

                        // Update gun position and rotation
                        let gun_distance_from_center = PLAYER_SIZE / 2.0 + 10.0; // half player size plus a bit more
                        let gun_offset = Vec3::new(
                            gun_distance_from_center * angle.cos(),
                            gun_distance_from_center * angle.sin(),
                            0.0,
                        );
                        gun_transform.translation = gun_offset;
                        gun_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

                        // Shooting logic
                        if mouse_button_input.just_pressed(MouseButton::Left)
                            && gun.shoot_timer <= 0.0
                        {
                            gun.shoot_timer = gun.shoot_cooldown; // reset the firing cooldown timer
                            spawn_bullet(
                                &mut commands,
                                player_transform.translation + gun_transform.translation,
                                gun_transform.rotation,
                                diff,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn spawn_bullet(commands: &mut Commands, bullet_origin: Vec3, rotation: Quat, direction: Vec2) {
    let bullet_direction = direction.normalize();

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: bullet_origin,
                rotation,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.0),
                custom_size: Some(Vec2::new(20.0, 2.0)),
                ..default()
            },
            ..default()
        })
        .insert(Bullet {
            lifetime: BULLET_LIFETIME,
            speed: BULLET_SPEED,
            direction: bullet_direction,
        });
}
