use crate::game::spell_system::storage::SpellPool;
use bevy::app::{App, Startup};
use bevy::prelude::ResMut;

pub mod cores;
pub mod modifiers;
pub mod multicasters;
pub mod targeters;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_spells);
}

pub fn load_spells(mut pool: ResMut<SpellPool>) {
    pool.insert_spells(cores::get_spells());
    pool.insert_spells(modifiers::get_spells());
    pool.insert_spells(multicasters::get_spells());
    pool.insert_spells(targeters::get_spells());
}
