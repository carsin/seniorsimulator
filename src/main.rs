use bevy::{prelude::*, window::PrimaryWindow};

const BULLET_SPEED: f32 = 1500.0;
const BULLET_LIFETIME: f32 = 3.0;
const GRID_SIZE: f32 = 1000.0; // Grid boundary
const GRID_WIDTH: f32 = 2.0;  // Width of the grid lines

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, player_movement_sys)
        .add_systems(Update, camera_follow_player_system)
        .add_systems(Update, bullet_movement_system)
        .add_systems(Update, gun_controls)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct MousePosition(Vec2);

#[derive(Component)]
struct GunController {
    shoot_cooldown: f32,
    shoot_timer: f32,
}

#[derive(Component)]
struct Bullet {
    lifetime: f32,
    speed: f32,
    direction: Vec2,
}

fn setup(mut commands: Commands) {
    // setup camera
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle).insert(MainCamera);
    
    // mouse pos resource
    commands.insert_resource(MousePosition(Vec2::ZERO));

    draw_grid(&mut commands);

    // spawn player box
    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.5),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(GunController {
            shoot_cooldown: 0.2,
            shoot_timer: 0.0,
        });
}

fn gun_controls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&GunController, &Transform), With<Player>>,
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().expect("Primary window not found");
    let (camera, camera_transform) = camera_query.get_single().expect("Main camera not found");

    if let Ok((gun_controller, player_transform)) = query.get_single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            // Convert cursor position from screen space to world space
            if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position).map(|ray| ray.origin.truncate()) {
                let diff = world_position - Vec2::new(player_transform.translation.x, player_transform.translation.y);
                let angle = diff.y.atan2(diff.x);

                if mouse_button_input.just_pressed(MouseButton::Left) {
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

fn bullet_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Bullet, &mut Transform)>,
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
            commands.entity(entity).insert(Bullet {
                lifetime: bullet.lifetime - time.delta_seconds(),
                ..*bullet
            });
        }
    }
}

fn player_movement_sys(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let speed = 300.0;

    let mut direction = Vec3::ZERO;

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

    player_transform.translation += time.delta_seconds() * direction * speed;
}

// System to make the camera follow the player
// fn camera_follow_player_system(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (With<Camera2d>, With<CameraFollow>)>,
// ) {
//     let player_transform = player_query.single();
//     let mut camera_transform = camera_query.single_mut();
//     camera_transform.translation.x = player_transform.translation.x;
//     camera_transform.translation.y = player_transform.translation.y;
// }

fn draw_grid(commands: &mut Commands) {
    let map_size = (GRID_SIZE / GRID_WIDTH) as u32;

    // Horizontal lines
    for i in 0..=map_size {
        let position = i as f32 * GRID_WIDTH - GRID_SIZE / 2.0;
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, position, 0.0)),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_SIZE, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });

        // Vertical lines
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(position, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, GRID_SIZE)),
                ..default()
            },
            ..default()
        });
    }
}

fn camera_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
