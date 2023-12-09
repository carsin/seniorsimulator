use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod constants;
mod game;
mod gun;
mod input;
mod npc;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                player::player_movement_sys,
                player::camera_follow_player_system,
                game::bullet_collision_system,
                gun::gun_controls,
                npc::npc_movement_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // remove rapier's default gravity
    rapier_config.gravity = Vec2::ZERO;

    // setup camera
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle).insert(player::MainCamera);

    // draw background
    game::draw_grid(&mut commands);

    // spawn player
    player::spawn_player(&mut commands);

    // Spawn some NPCs
    for _ in 0..10 {
        npc::spawn_npc(&mut commands);
    }
}

// fn bullet_movement_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &gun::Bullet, &mut Transform)>,
// ) {
//     for (entity, bullet, mut transform) in query.iter_mut() {
//         let movement = bullet.direction * bullet.speed * time.delta_seconds();
//         transform.translation.x += movement.x;
//         transform.translation.y += movement.y;
//
//         // Decrease lifetime and despawn if lifetime is over
//         if bullet.lifetime <= 0.0 {
//             commands.entity(entity).despawn();
//         } else {
//             // Update the lifetime of the bullet
//             commands.entity(entity).insert(gun::Bullet {
//                 lifetime: bullet.lifetime - time.delta_seconds(),
//                 ..*bullet
//             });
//         }
//     }
// }
