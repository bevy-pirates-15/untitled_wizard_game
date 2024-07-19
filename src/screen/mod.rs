//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod pause;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>().add_sub_state::<GameState>();
    app.enable_state_scoped_entities::<Screen>();
    app.enable_state_scoped_entities::<GameState>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
        pause::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen { 
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
}

/// The game's states while playing
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Running,
    Paused
}