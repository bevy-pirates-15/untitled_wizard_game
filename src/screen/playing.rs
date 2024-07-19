//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::{GameState, Screen};
use crate::game::{audio::soundtrack::Soundtrack, spawn::level::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        toggle_game_pause
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands.trigger(Soundtrack::Gameplay);
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(Soundtrack::Disable);
}

fn toggle_game_pause(
    curr_pause_state: Res<State<GameState>>,
    mut next_pause_state: ResMut<NextState<GameState>>,
) {
    match curr_pause_state.get() {
        GameState::Paused => next_pause_state.set(GameState::Running),
        GameState::Running => next_pause_state.set(GameState::Paused),
    }
}
