/// Core game plugin - Main gameplay systems

use bevy::prelude::*;
use crate::components::{Player, Position, Renderable, Viewshed, Health, CombatStats, Name};
use crate::resources::{CurrentMap, VisibilityMap, PlayerActionPoints, CombatLog};
use crate::systems::{
    player_input_system, apply_movement_system, camera_follow_system,
    calculate_fov_system, update_visibility_map_system,
    apply_tile_visibility_system, hide_entities_outside_fov_system,
    check_turn_end_system, start_player_turn_system, enemy_turn_system,
    enemy_action_system,
    player_attack_input_system, execute_attack_system, check_player_death_system,
    spawn_enemies_system,
    PendingAttack,
    MapTile, TileBaseColor,
};
use crate::systems::movement::PendingMovement;
use crate::states::{GameState, TurnState};
use crate::constants::*;

/// Resource to track if game has been initialized
#[derive(Resource, Default)]
struct GameInitialized(bool);

/// Main game plugin that sets up the core gameplay
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            // State management
            .init_state::<TurnState>()
            // Resources
            .init_resource::<PendingMovement>()
            .init_resource::<GameInitialized>()
            .init_resource::<VisibilityMap>()
            .init_resource::<PlayerActionPoints>()
            .init_resource::<PendingAttack>()
            .init_resource::<CombatLog>()
            // One-time setup when first entering Playing state
            .add_systems(OnEnter(GameState::Playing), initialize_game)
            // Player turn systems (run during Playing AND PlayerTurn state)
            .add_systems(Update, (
                // Input capture runs every frame (responsive feel)
                player_input_system,
                player_attack_input_system,
                // Action execution
                apply_movement_system,
                execute_attack_system,
                camera_follow_system,
                update_sprite_positions,
                // FOV systems (run after movement)
                calculate_fov_system,
                update_visibility_map_system,
                apply_tile_visibility_system,
                hide_entities_outside_fov_system,
                // Check if turn should end or player died
                check_turn_end_system,
                check_player_death_system,
            ).chain().run_if(in_state(GameState::Playing).and(in_state(TurnState::PlayerTurn))))
            // Enemy turn systems
            .add_systems(Update, (
                enemy_action_system,
                enemy_turn_system,
            ).chain().run_if(in_state(GameState::Playing).and(in_state(TurnState::EnemyTurn))))
            // Turn transition events
            .add_systems(OnEnter(TurnState::PlayerTurn), start_player_turn_system);
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
            let base_color = match tile_type {
                crate::resources::TileType::Floor => COLOR_FLOOR,
                crate::resources::TileType::Wall => COLOR_WALL,
            };

            commands.spawn((
                MapTile {
                    position: Position::new(x as i32, y as i32),
                },
                TileBaseColor(base_color),
                Sprite {
                    color: Color::srgb(0.0, 0.0, 0.0), // Start black (unseen)
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
    info!("Map tiles rendered with FOV support!");

    // Spawn player
    commands.spawn((
        Player,
        Position::new(10, 10),
        Name::new("Hero"),
        Health::new(PLAYER_STARTING_HEALTH),
        CombatStats::new(PLAYER_ATTACK_POWER, PLAYER_DEFENSE),
        Renderable::new(COLOR_PLAYER),
        Viewshed::new(FOV_RADIUS),
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
    info!("Player spawned at (10, 10) with {} HP and FOV radius {}",
          PLAYER_STARTING_HEALTH, FOV_RADIUS);

    // Spawn enemies (before inserting map resource)
    spawn_enemies_system(commands.reborrow(), &map);

    // Now insert the map resource
    commands.insert_resource(map);
    info!("Map created!");

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
