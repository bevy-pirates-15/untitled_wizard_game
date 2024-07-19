//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod pause;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<PauseState>();
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
    app.enable_state_scoped_entities::<PauseState>();

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
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PauseState {
    #[default]
    Paused,
    Running,
}
