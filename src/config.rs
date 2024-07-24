use std::ops::Range;

// Map
pub const MAP_HEIGHT: f32 = 5000.0;
pub const MAP_WIDTH: f32 = 5000.0;

// Borders
pub const BORDER_THICKNESS: f32 = 100.0;

// Player
pub const PLAYER_SPEED: f32 = 21000.;
pub const PLAYER_HEALTH: f32 = 5.;

// Enemy
pub const MAX_ENEMIES: usize = 1500;
pub const ENEMY_SPEED: f32 = 0.75;
pub const ENEMY_HEALTH: f32 = 75.0;
// pub const ENEMY_DAMAGE: f32 = 10.0;
pub const SPAWN_RATE_PER_SECOND: usize = 200;
pub const SPAWN_RADIUS: Range<f32> = 1000.0..4000.0;
pub const ENEMY_SPAWN_PERIOD: f32 = 1.0;

// Experience Mechanic
pub const BASE_ENEMY_XP: u32 = 5;
