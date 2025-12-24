/// Game state management for Bevy state machine

use bevy::prelude::*;

/// Main game states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Main menu screen
    #[default]
    MainMenu,

    /// Active gameplay
    Playing,

    /// Game paused (press ESC)
    Paused,

    /// Player died (permadeath)
    GameOver,

    /// Player won (optional victory condition)
    Victory,
}

/// Turn state within gameplay
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TurnState {
    /// Waiting for player input
    #[default]
    PlayerTurn,

    /// Processing enemy actions
    EnemyTurn,

    /// Transitioning between turns
    TurnTransition,
}

/// Plugin to manage game states
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_state::<TurnState>()
            .add_systems(Update, handle_pause_input);
    }
}

/// Handle pause toggle with ESC key
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}
