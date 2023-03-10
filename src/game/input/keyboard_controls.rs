use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use iyes_loopless::state::CurrentState;

use crate::game::collision_debug::HitCirclesVisualizationConfig;
use crate::game::game_state::GameState;
use crate::game::sprites_debug::SpritesBoundariesConfig;
use crate::game::PlayerMovement;

pub struct GameKeyboardControlsPlugin;

impl Plugin for GameKeyboardControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_keyboard_input);
    }
}

// TODO: handle a case of multiple arrows pressed at once
fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_movement_query: Query<&mut PlayerMovement>,
    mut commands: Commands,
    current_state: Res<CurrentState<GameState>>,
    sprites_boundaries_config: Option<ResMut<SpritesBoundariesConfig>>,
    hit_circles_visualization_config: Option<ResMut<HitCirclesVisualizationConfig>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Left;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Right;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Up;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Down;
        }
    }

    // d = enter [d]ebug pause
    #[cfg(debug_assertions)]
    if keyboard_input.just_pressed(KeyCode::D) {
        match *current_state {
            CurrentState(GameState::InGame) => {
                commands.insert_resource(NextState(GameState::DebugPause));
            },
            CurrentState(GameState::DebugPause) => {
                commands.insert_resource(NextState(GameState::InGame));
            },
            _ => {},
        };
    }
    #[cfg(debug_assertions)]
    if let CurrentState(GameState::DebugPause) = *current_state {
        if keyboard_input.just_pressed(KeyCode::Period) {
            commands.insert_resource(NextState(GameState::DebugResumeFor1Frame));
        }
    }
    // s = draw [s]prite boundaries
    #[cfg(debug_assertions)]
    if keyboard_input.just_pressed(KeyCode::S) {
        if let Some(mut config) = sprites_boundaries_config {
            config.is_plugin_enabled = !config.is_plugin_enabled;
        }
    }
    // c = draw hit [c]ircle visualization
    #[cfg(debug_assertions)]
    if keyboard_input.just_pressed(KeyCode::C) {
        if let Some(mut config) = hit_circles_visualization_config {
            config.is_plugin_enabled = !config.is_plugin_enabled;
        }
    }
}
