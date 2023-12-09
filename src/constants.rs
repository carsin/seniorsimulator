use bevy::prelude::*;

pub const BULLET_SPEED: f32 = 2000.0;
pub const BULLET_LIFETIME: f32 = 3.0;
pub const BULLET_SIZE: Vec2 = Vec2 { x: 15., y: 2. };
pub const GUN_OFFSET: f32 = 8.;
pub const PLAYER_SIZE: f32 = 30.0;
pub const PLAYER_MAX_SPEED: f32 = 400.0;
pub const GRID_SIZE: f32 = 2000.0; // Grid boundary
pub const GRID_WIDTH: f32 = 2.0; // Width of the grid lines
pub const MAP_SIZE: u32 = 40; // Number of grid lines
pub const FRICTION: f32 = 10.;
pub const PLAYER_ACCEL: f32 = 600.0 * FRICTION;
pub const NPC_SPEED: f32 = 100.0;
pub const NPC_INITIAL_HEALTH: f32 = 100.; // Initial health of the NPC
