use std::sync::Arc;

use bevy::prelude::*;

use crate::game::assets::{ImageAsset, ImageAssets};
use crate::game::spell_system::storage::RebuildWand;
use crate::game::spell_system::triggers::PlayerSpellTrigger;
use crate::game::spell_system::SpellModifierNode;
use crate::{
    game::{
        player_mods::aiming::{AttachToPlayer, PlayerAim},
        spell_system::casting::{CasterTargeter, SequentialCaster, SpellCastValues, SpellCaster},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_wand);
}

#[derive(Event, Debug)]
pub struct SpawnWand;

#[derive(Component, Debug, Default)]
pub struct Wand;

fn spawn_wand(_trigger: Trigger<SpawnWand>, images: Res<ImageAssets>, mut commands: Commands) {
    let mut e = commands.spawn((
        Name::new("Wand"),
        Wand,
        SpriteBundle {
            texture: images[&ImageAsset::Wand].clone_weak(),
            ..default()
        },
        PlayerAim(Vec2::new(0.0, 1.0)),
        StateScoped(Screen::Playing),
        AttachToPlayer {
            origin_offset: Vec3::new(0., -3.0, 0.1),
        },
    ));

    // wand_inventory.rebuild_effects();
    e.insert((
        SpellCaster::Sequential(SequentialCaster::new()),
        PlayerSpellTrigger {
            values: SpellCastValues {
                spread: 10.0,
                modifiers: Arc::new(SpellModifierNode::Root),
            },
            spells: Arc::new(vec![]),
        },
        CasterTargeter::RotationBased(Vec2::new(0.0, 1.0)),
    ));

    commands.trigger(RebuildWand);
}
