use bevy::{prelude::*, window::PrimaryWindow};

const BULLET_SPEED: f32 = 1000.0;
const BULLET_LIFETIME: f32 = 10.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, player_movement_sys)
        .add_systems(Update, bullet_movement_system)
        .add_systems(Update, gun_controls)
        .run();
}

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
    commands.spawn(camera_bundle);
    
    // mouse pos resource
    commands.insert_resource(MousePosition(Vec2::ZERO));

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
            shoot_cooldown: 0.01,
            shoot_timer: 0.0,
        });
}

fn gun_controls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&GunController, &Transform), With<Player>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let window = window_query.get_single().expect("Primary window not found");
    if let Ok((gun_controller, player_transform)) = query.get_single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            // convert cursor position from screen space to world space
            let window_size = Vec2::new(window.width(), window.height());
            let mut cursor_world_position = cursor_position - window_size / 2.0;
            cursor_world_position.y *= -1.0; // invert y-axis to match world space

            let diff = cursor_world_position - Vec2::new(player_transform.translation.x, player_transform.translation.y);
            let angle = diff.y.atan2(diff.x);

            if mouse_button_input.just_pressed(MouseButton::Left) {
                let mut spawn_transform = Transform::from_translation(player_transform.translation);
                spawn_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);

                commands.spawn(SpriteBundle {
                    transform: spawn_transform,
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.0),
                        custom_size: Some(Vec2::new(20.0, 2.0)),
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

fn mouse_position_system(
    mut mouse_position: ResMut<MousePosition>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.read() {
        mouse_position.0 = event.position;
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
