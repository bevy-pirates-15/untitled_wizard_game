use bevy::prelude::*;

pub mod aiming;
pub mod damage;
pub mod health;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        movement::plugin,
        aiming::plugin,
        damage::plugin,
        health::plugin,
    ));
}
