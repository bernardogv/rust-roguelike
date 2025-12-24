/// Game constants and configuration values

use bevy::prelude::*;

// Window settings
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_TITLE: &str = "Roguelike Dungeon Crawler";

// Tile and grid settings
pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 45;

// Camera settings
pub const CAMERA_SCALE: f32 = 1.0;
pub const CAMERA_FOLLOW_SPEED: f32 = 5.0;

// Game balance
pub const PLAYER_STARTING_HEALTH: i32 = 100;
pub const PLAYER_STARTING_ACTION_POINTS: i32 = 1;
pub const MOVEMENT_ACTION_COST: i32 = 1;
pub const FOV_RADIUS: i32 = 8;

// Combat settings
pub const ATTACK_ACTION_COST: i32 = 1;
pub const BASE_HIT_CHANCE: u32 = 75;
pub const DAMAGE_VARIANCE: i32 = 2;

// Player combat stats
pub const PLAYER_ATTACK_POWER: i32 = 10;
pub const PLAYER_DEFENSE: i32 = 2;

// Enemy combat stats
pub const ENEMY_STARTING_HEALTH: i32 = 30;
pub const ENEMY_ATTACK_POWER: i32 = 8;
pub const ENEMY_DEFENSE: i32 = 1;
pub const ENEMY_FOV_RADIUS: i32 = 6;
pub const ENEMY_MIN_COUNT: usize = 3;
pub const ENEMY_MAX_COUNT: usize = 5;

// Colors (brightened significantly for visibility against black background)
pub const COLOR_FLOOR: Color = Color::srgb(0.7, 0.7, 0.8);  // Bright blue-gray floor
pub const COLOR_WALL: Color = Color::srgb(0.9, 0.8, 0.7);   // Bright tan walls
pub const COLOR_PLAYER: Color = Color::srgb(0.0, 0.9, 0.0); // Bright green player
pub const COLOR_ENEMY: Color = Color::srgb(0.9, 0.0, 0.0);  // Bright red enemies
pub const COLOR_FOV_VISIBLE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_FOV_EXPLORED: Color = Color::srgb(0.5, 0.5, 0.5);
pub const COLOR_FOV_UNSEEN: Color = Color::srgb(0.0, 0.0, 0.0);

// UI Colors
pub const COLOR_UI_TEXT: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_UI_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
pub const COLOR_HEALTH_BAR_FULL: Color = Color::srgb(0.0, 0.8, 0.0);
pub const COLOR_HEALTH_BAR_LOW: Color = Color::srgb(0.8, 0.0, 0.0);

// Z-layers for rendering order
pub const Z_LAYER_FLOOR: f32 = 0.0;
pub const Z_LAYER_ITEMS: f32 = 1.0;
pub const Z_LAYER_CHARACTERS: f32 = 2.0;
pub const Z_LAYER_UI: f32 = 10.0;
