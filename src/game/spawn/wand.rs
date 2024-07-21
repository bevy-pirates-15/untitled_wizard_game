use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::{
    color::palettes::css::BROWN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{game::aiming::PlayerAim, screen::Screen};
use crate::game::spells::{SpellEffect, SpellModifierNode};
use crate::game::spells::casting::{
    PlayerSpellTrigger, SequentialCaster, SpellCastContext, SpellCastEvent,
};
use crate::game::spells::examples::{TriggerSpell, ZapSpell};
use crate::game::spells::triggers::ToTrigger;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_wand);
}

#[derive(Event, Debug)]
pub struct SpawnWand;

#[derive(Component, Debug, Default)]
pub struct Wand;

fn spawn_wand(
    _trigger: Trigger<SpawnWand>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut e = commands.spawn((
        Name::new("Wand"),
        Wand,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
            transform: Transform::default().with_scale(Vec2::new(20., 70.).extend(2.0)),
            material: materials.add(Color::from(BROWN)),
            ..default()
        },
        PlayerAim::default(),
        StateScoped(Screen::Playing),
    ));

    let wand_spell_context = SpellCastContext {
        caster: e.id(),
        spell_delay: Arc::new(Mutex::new(Duration::from_secs_f32(0.2))),
        spread: 0.0,
        modifiers: Arc::new(SpellModifierNode::Root),
    };

    let wand_spells: Arc<Vec<Arc<dyn SpellEffect>>> = Arc::new(vec![
        Arc::new(ZapSpell { base_damage: 1.0 }),
        Arc::new(ZapSpell { base_damage: 2.0 }),
        Arc::new(TriggerSpell {
            trigger_spell: Arc::new(ZapSpell { base_damage: 3.0 }),
            spells_triggered: Arc::new(vec![
                Arc::new(ZapSpell { base_damage: 4.0 }),
                Arc::new(ZapSpell { base_damage: 5.0 }),
            ]),
        }),
    ]);

    e.insert((
        SequentialCaster::new(),
        PlayerSpellTrigger {
            to_trigger: ToTrigger::new(wand_spells, wand_spell_context),
        },
    ));

    e.observe(
        |trigger: Trigger<SpellCastEvent>, mut q: Query<&mut SequentialCaster>| {
            println!("Wand triggered");
            let Ok(mut seq_caster) = q.get_mut(trigger.entity()) else {
                println!("Failed to get sequential caster");
                return;
            };

            seq_caster.try_cast(trigger.event().to_trigger.clone());
        },
    );
}
