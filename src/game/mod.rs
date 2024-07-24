//! Game mechanics and content.

use bevy::prelude::*;

mod aiming;
mod animation;
pub mod assets;
pub mod audio;
mod camera;
mod enemy;
pub mod input;
pub mod levelling;
mod movement;
pub mod physics;
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
        spells::plugin,
        levelling::plugin,
        projectiles::plugin,
    ));

    app.register_type::<Health>();
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ItemDrop;
