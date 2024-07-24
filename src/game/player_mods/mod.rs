use bevy::prelude::*;

pub mod aiming;
pub mod movement;
mod damage;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, aiming::plugin, damage::plugin));
}
