/// Core game plugin - Main gameplay systems

use bevy::prelude::*;
use crate::components::{Player, Position, Renderable};
use crate::resources::CurrentMap;
use crate::systems::{player_input_system, apply_movement_system, camera_follow_system};
use crate::systems::movement::PendingMovement;
use crate::states::GameState;
use crate::constants::*;

/// Resource to track if game has been initialized
#[derive(Resource, Default)]
struct GameInitialized(bool);

/// Main game plugin that sets up the core gameplay
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<PendingMovement>()
            .init_resource::<GameInitialized>()
            // One-time setup when first entering Playing state
            .add_systems(OnEnter(GameState::Playing), initialize_game)
            // Update systems (run during Playing state)
            .add_systems(Update, (
                player_input_system,
                apply_movement_system,
                camera_follow_system,
                update_sprite_positions,
            ).run_if(in_state(GameState::Playing)));
    }
}

/// Initialize the game only once
fn initialize_game(
    mut commands: Commands,
    mut initialized: ResMut<GameInitialized>,
) {
    // Only initialize once
    if initialized.0 {
        info!("Game already initialized, skipping setup");
        return;
    }

    // Create map
    let map = CurrentMap::test_map();

    // Spawn map tiles first (before inserting resource)
    for y in 0..map.height {
        for x in 0..map.width {
            let tile_type = map.tiles[y][x];
            let color = match tile_type {
                crate::resources::TileType::Floor => COLOR_FLOOR,
                crate::resources::TileType::Wall => COLOR_WALL,
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    Z_LAYER_FLOOR,
                ),
            ));
        }
    }
    info!("Map tiles rendered!");

    // Now insert the map resource
    commands.insert_resource(map);
    info!("Map created!");

    // Spawn player
    commands.spawn((
        Player,
        Position::new(10, 10),
        Renderable::new(COLOR_PLAYER),
        Sprite {
            color: COLOR_PLAYER,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(
            10.0 * TILE_SIZE,
            10.0 * TILE_SIZE,
            Z_LAYER_CHARACTERS,
        ),
    ));
    info!("Player spawned at (10, 10)");

    // Mark as initialized
    initialized.0 = true;
}

/// Update sprite positions based on grid Position component
fn update_sprite_positions(
    mut query: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = pos.x as f32 * TILE_SIZE;
        transform.translation.y = pos.y as f32 * TILE_SIZE;
    }
}
