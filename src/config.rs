use std::ops::Range;

// Map
pub const MAP_HEIGHT: f32 = 5000.0;
pub const MAP_WIDTH: f32 = 5000.0;

// Borders
pub const BORDER_THICKNESS: f32 = 100.0;

// Player
pub const PLAYER_SPEED: f32 = 5000.;
pub const PLAYER_HEALTH: f32 = 9.99;

// Enemy
pub const MAX_ENEMIES: usize = 1500;
pub const ENEMY_SPEED: f32 = 40.;
pub const ENEMY_HEALTH: f32 = 75.0;
// pub const ENEMY_DAMAGE: f32 = 10.0;
pub const SPAWN_RATE_PER_SECOND: usize = 4;
pub const SPAWN_RADIUS: Range<f32> = 300.0..600.0;
pub const ENEMY_SPAWN_PERIOD: f32 = 1.0;

// Experience Mechanic
pub const BASE_ENEMY_XP: u32 = 5;
pub const EXPERIENCE_SPEED: f32 = 100.;
pub const EXPERIENCE_RADIUS: f32 = 200.;
