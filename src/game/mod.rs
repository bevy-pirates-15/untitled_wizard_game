//! Game mechanics and content.

use bevy::prelude::*;

mod aiming;
mod animation;
pub mod assets;
pub mod audio;
mod camera;
mod enemy;
pub mod input;
mod movement;
pub mod projectiles;
pub mod spawn;
pub mod spells;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        input::plugin,
        animation::plugin,
        audio::plugin,
        movement::plugin,
        spawn::plugin,
        aiming::plugin,
        enemy::plugin,
    ));
}
