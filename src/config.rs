use std::ops::Range;

// Map
pub const MAP_HEIGHT: f32 = 768.0;
pub const MAP_WIDTH: f32 = 768.0;

// Borders
pub const BORDER_THICKNESS: f32 = 100.0;

// Player
pub const PLAYER_SPEED: f32 = 5000.;
pub const PLAYER_HEALTH: f32 = 9.99;

// Enemy
pub const ENEMY_SPEED: f32 = 20.0;
pub const ENEMY_HEALTH: f32 = 75.0;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const SPAWN_RADIUS: Range<f32> = 600.0..700.0;
pub const RANGED_ENEMY_DIST: u32 = 50;

// Experience Mechanic
pub const BASE_ENEMY_XP: u32 = 5;
pub const EXPERIENCE_SPEED: f32 = 200.;
pub const EXPERIENCE_RADIUS: f32 = 50.;
