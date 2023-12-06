use bevy::{prelude::*, window::PrimaryWindow};

const BULLET_SPEED: f32 = 1500.0;
const BULLET_LIFETIME: f32 = 3.0;
const PLAYER_SIZE: f32 = 30.0;
const PLAYER_ACCEL: f32 = 30.0;
const PLAYER_MAX_SPEED: f32 = 380.0;
const GRID_SIZE: f32 = 2000.0; // Grid boundary
const GRID_WIDTH: f32 = 2.0;  // Width of the grid lines
const MAP_SIZE: u32 = 40;     // Number of grid lines
const FRICTION: f32 = 0.92;

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
struct Player {
    velocity: Vec3,
    max_velocity: f32,
    acceleration: f32,
}

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
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
            ..Default::default()
        })
        .insert(Player {
            velocity: Vec3::ZERO,
            max_velocity: PLAYER_MAX_SPEED,
            acceleration: PLAYER_ACCEL,
        })
        .insert(GunController {
            shoot_cooldown: 0.2,
            shoot_timer: 0.0,
        });
}

fn gun_controls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&mut GunController, &Transform), With<Player>>,
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

fn camera_follow_player_system(
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
