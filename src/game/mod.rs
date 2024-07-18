//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod input;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        input::plugin,
        animation::plugin,
        audio::plugin,
        movement::plugin,
        spawn::plugin,
    ));
}
