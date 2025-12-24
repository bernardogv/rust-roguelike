/// Main entry point for the Roguelike Dungeon Crawler
///
/// This game is built using Bevy's Entity Component System (ECS) architecture.
/// The game follows a turn-based roguelike pattern with procedural generation.

use bevy::prelude::*;
use bevy::window::{WindowResolution, PresentMode};
use bevy::render::camera::ClearColorConfig;
use rust_roguelike::{constants::*, StatesPlugin, GameCorePlugin, GameState};

fn main() {
    App::new()
        // Bevy default plugins with custom window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Custom game plugins
        .add_plugins((StatesPlugin, GameCorePlugin))
        // Setup systems
        .add_systems(Startup, setup)
        // Run the game
        .run();
}

/// Initial setup - runs once at startup
fn setup(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Spawn 2D camera using new Bevy 0.15 API
    commands.spawn((
        Camera2d,
        Camera {
            // Set black background for better tile visibility
            clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    // Transition to Playing state to start the game
    next_state.set(GameState::Playing);

    info!("Roguelike game initialized!");
    info!("Use WASD or Arrow keys to move");
    info!("Press ESC to pause/unpause");
}
