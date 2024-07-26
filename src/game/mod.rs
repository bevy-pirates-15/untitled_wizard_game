//! Game mechanics and content.

use crate::game::projectiles::ProjectileTeam;
use bevy::prelude::*;
use std::time::Duration;

mod animation;
pub mod assets;
pub mod audio;
mod camera;
pub mod enemy;
pub mod input;
pub mod levelling;
pub mod physics;
pub mod player_mods;
pub mod projectiles;
pub mod spawn;
pub mod spell_system;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        input::plugin,
        animation::plugin,
        audio::plugin,
        spawn::plugin,
        enemy::plugin,
        spell_system::plugin,
        levelling::plugin,
        projectiles::plugin,
        physics::plugin,
        player_mods::plugin,
    ));

    app.register_type::<Damageable>();
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Damageable {
    pub max_health: f32,
    pub health: f32,
    pub team: ProjectileTeam,
    pub invincibility_timer: Duration,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ItemDrop;
