use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub is_playing: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self { is_playing: true }
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_controls);
    }
}

fn pause_controls(keyboard_input: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.is_playing = !game_state.is_playing;
    }
}

fn run_if_gamestate_is_playing(game_state: Res<GameState>) -> bool {
    game_state.is_playing
}
