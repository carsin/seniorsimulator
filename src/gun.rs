use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn gun_controls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<player::MainCamera>>,
    mut player_query: Query<(&mut Transform, &Children), (With<player::Player>, Without<Gun>)>,
    mut gun_query: Query<&mut Gun, With<Gun>>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().expect("Primary window not found");
    let (camera, camera_transform) = camera_query.get_single().expect("Main camera not found");

    for (mut player_transform, children) in player_query.iter_mut() {
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

                // rotate player towards cursor
                player_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

                if let Some(gun_entity) = children
                    .iter()
                    .find(|&&entity| gun_query.get_component::<Gun>(entity).is_ok())
                {
                    if let Ok(mut gun) = gun_query.get_mut(*gun_entity) {
                        gun.shoot_timer -= time.delta_seconds();

                        if (mouse_button_input.just_pressed(MouseButton::Left)
                            || keyboard_input.pressed(KeyCode::Space))
                            && gun.shoot_timer <= 0.0
                        {
                            gun.shoot_timer = gun.shoot_cooldown;

                            // calculate the bullet origin using the player's direction
                            let bullet_offset = Vec3::new(angle.cos(), angle.sin(), 0.0)
                                * (PLAYER_SIZE / 2.0 + GUN_OFFSET);
                            let bullet_origin = player_transform.translation + bullet_offset;

                            // spawn bullet
                            spawn_bullet(
                                &mut commands,
                                bullet_origin,
                                player_transform.rotation,
                                diff.normalize(),
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
                custom_size: Some(BULLET_SIZE),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(BULLET_SIZE.x / 2., BULLET_SIZE.y / 2.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: bullet_direction * BULLET_SPEED,
            angvel: 0.0,
        })
        .insert(Bullet {
            lifetime: BULLET_LIFETIME,
            speed: BULLET_SPEED,
            direction: bullet_direction,
        });
}
