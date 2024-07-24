use std::sync::Arc;

use bevy::{
    color::palettes::css::BROWN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::game::spells::casting::{
    CasterTargeter, SequentialCaster, SpellCastValues, SpellCaster,
};
use crate::game::spells::examples::{TriggerSpell, ZapSpell};
use crate::game::spells::triggers::PlayerSpellTrigger;
use crate::game::spells::{SpellEffect, SpellModifierNode};
use crate::{game::aiming::PlayerAim, screen::Screen};

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
            mesh: Mesh2dHandle(
                meshes.add(
                    Rectangle::new(20., 70.)
                        .mesh()
                        .build()
                        .translated_by(Vec3::new(0.0, 35.0, 0.0)),
                ),
            ),
            // transform: Transform::default().with_scale(Vec2::new(20., 70.).extend(2.0)),
            material: materials.add(Color::from(BROWN)),
            ..default()
        },
        PlayerAim(Vec2::new(0.0, 1.0)),
        StateScoped(Screen::Playing),
    ));

    // let wand_spell_context = SpellCastContext {
    //     caster: e.id(),
    //     spell_vec: Vec2::new(0.0, 1.0),
    //     spell_delay: Arc::new(Mutex::new(Duration::from_secs_f32(0.2))),
    //     spread: 0.0,
    //     modifiers: Arc::new(SpellModifierNode::Root),
    // };

    let wand_spells: Arc<Vec<Arc<dyn SpellEffect>>> = Arc::new(vec![
        Arc::new(ZapSpell { base_damage: 81.0 }),
        Arc::new(ZapSpell { base_damage: 82.0 }),
        Arc::new(TriggerSpell {
            trigger_spell: Arc::new(ZapSpell { base_damage: 83.0 }),
            spells_triggered: Arc::new(vec![
                Arc::new(ZapSpell { base_damage: 84.0 }),
                Arc::new(ZapSpell { base_damage: 85.0 }),
            ]),
        }),
        Arc::new(ZapSpell { base_damage: 86.0 }),
    ]);

    e.insert((
        SpellCaster::Sequential(SequentialCaster::new()),
        PlayerSpellTrigger {
            values: SpellCastValues {
                spread: 10.0,
                modifiers: Arc::new(SpellModifierNode::Root),
            },
            spells: wand_spells,
        },
        CasterTargeter::RotationBased(Vec2::new(0.0, 1.0)),
    ));
}
