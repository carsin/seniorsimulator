use bevy::prelude::*;
use super::constants::*;

#[derive(Component)]
pub struct Npc {
    pub velocity: Vec3,
    pub speed: f32,
    pub health: f32,
}

pub fn npc_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Npc)>,
) {
    for (mut transform, mut npc) in query.iter_mut() {
        // Randomly decide whether to change direction
        if rand::random::<f32>() < 0.05 { //
            npc.velocity = Vec3::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5, 0.0).normalize_or_zero() * npc.speed;
        }

        // Update position and clamp within grid
        transform.translation += npc.velocity * time.delta_seconds();
        transform.translation.x = transform.translation.x.clamp(-GRID_SIZE / 2.0, GRID_SIZE / 2.0);
        transform.translation.y = transform.translation.y.clamp(-GRID_SIZE / 2.0, GRID_SIZE / 2.0);
    }
}
