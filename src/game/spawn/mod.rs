//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod borders;
pub mod map;
pub mod player;
pub mod prompt;
pub mod wand;
// pub mod lighting;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map::plugin,
        player::plugin,
        wand::plugin,
        borders::plugin,
        prompt::plugin,
        // lighting::plugin,
    ));
}
