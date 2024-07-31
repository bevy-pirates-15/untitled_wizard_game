use std::ops::Range;

// Map
pub const MAP_HEIGHT: f32 = 768.0;
pub const MAP_WIDTH: f32 = 768.0;

// Borders
pub const BORDER_THICKNESS: f32 = 100.0;

// Player
pub const PLAYER_SPEED: f32 = 5000.;
pub const PLAYER_HEALTH: f32 = 10.00;

// Enemy
pub const ENEMY_SPEED: f32 = 40.0;
pub const ENEMY_HEALTH: f32 = 55.0;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const SPAWN_RADIUS: Range<f32> = 400.0..500.0;
pub const RANGED_ENEMY_DIST: u32 = 200;

// Experience Mechanic
pub const BASE_ENEMY_XP: u32 = 10;
pub const EXPERIENCE_SPEED: f32 = 200.;
pub const EXPERIENCE_RADIUS: f32 = 50.;
