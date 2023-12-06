use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, player_movement_sys)
        .add_systems(Startup, setup)
        .run();
    println!("Hello, world!");
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    // setup camera
    commands.spawn(Camera2dBundle::default());

    // spawn player box
    commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.5),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn player_movement_sys(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let speed = 100.0;

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    player_transform.translation += time.delta_seconds() * direction * speed;
}
