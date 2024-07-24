//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod camera;
mod enemy;
pub mod input;
pub mod levelling;
pub mod physics;
pub mod player_mods;
pub mod projectiles;
pub mod spawn;
pub mod spells;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        input::plugin,
        animation::plugin,
        audio::plugin,
        spawn::plugin,
        enemy::plugin,
        spells::plugin,
        levelling::plugin,
        projectiles::plugin,
        physics::plugin,
        player_mods::plugin,
    ));

    app.register_type::<Health>();
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ItemDrop;
