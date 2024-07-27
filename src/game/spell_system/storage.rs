use std::sync::Arc;

use bevy::app::{App, Startup};
use bevy::prelude::{Event, IntoSystemConfigs, Query, ResMut, Resource, Trigger};
use log::{debug, info};

use crate::game::spell_system::{SpellComponent, SpellEffect};
use crate::game::spell_system::spells::load_spells;
use crate::game::spell_system::triggers::PlayerSpellTrigger;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpellPool>()
        .init_resource::<SpellInventory>()
        //temp system to load 6 random spells into the spell inventory
        .add_systems(
            Startup,
            (|mut spell_inventory: ResMut<SpellInventory>, pool: ResMut<SpellPool>| {
                // spell_inventory.push_spell(SpellComponent {
                //     data: Box::new(TriggerSpellData {
                //         spells_triggered: 1,
                //     }),
                //     icon_id: 24,
                // });
                // spell_inventory.push_spell(SpellComponent {
                //     data: Box::new(BangSpellData {
                //         base_damage: 40.0,
                //         radius: 50.0,
                //     }),
                //     icon_id: 3,
                // });
                // spell_inventory.push_spell(SpellComponent {
                //     data: Box::new(SplitterBoltsSpellData {
                //         base_damage: 20.0,
                //         projectile_count: 3,
                //     }),
                //     icon_id: 2,
                // });

                for _ in 0..2 {
                    spell_inventory.push_spell(pool.get_random_spell_component().clone());
                }
                spell_inventory.rebuild_effects()
            })
            .after(load_spells),
        )
        .observe(insert_spell_at_pos)
        .observe(rebuild_wand);
}

#[derive(Resource, Default)]
pub struct SpellPool {
    pub spells: Vec<SpellComponent>,
}
impl SpellPool {
    pub fn get_random_spell_component(&self) -> &SpellComponent {
        let index = rand::random::<usize>() % self.spells.len();
        &self.spells[index]
    }
    pub(crate) fn insert_spells(&mut self, spells: Vec<SpellComponent>) {
        self.spells.extend(spells);
    }
}

#[derive(Resource, Default)]
pub struct SpellInventory {
    pub spells: Vec<SpellComponent>,
    pub spell_effects: Vec<Arc<dyn SpellEffect>>,
}
impl SpellInventory {
    pub(crate) fn push_spell(&mut self, spell: SpellComponent) {
        self.spells.push(spell);
        self.rebuild_effects();
    }
    pub(crate) fn rebuild_effects(&mut self) {
        self.spell_effects.clear();

        let mut iter = self.spells.iter();
        while let Some(effect) = iter.next().and_then(|spell| spell.data.build(&mut iter)) {
            self.spell_effects.push(effect);
        }
    }
    pub(crate) fn insert_spell(&mut self, spell: SpellComponent, pos: SpellAddPos) {
        self.spells.insert(pos.get_index(&self.spells), spell);
        self.rebuild_effects();
    }
}
#[derive(Event)]
pub struct AddSpellTo(SpellComponent, SpellAddPos);
#[derive(Event)]
pub struct RebuildWand;

#[derive(Copy, Clone)]
pub enum SpellAddPos {
    #[allow(dead_code)]
    Start,
    #[allow(dead_code)]
    End,
    #[allow(dead_code)]
    Index(usize),
}
impl SpellAddPos {
    fn get_index(&self, spells: &[SpellComponent]) -> usize {
        match self {
            SpellAddPos::Start => 0,
            SpellAddPos::End => spells.len(),
            SpellAddPos::Index(i) => *i,
        }
    }
}

fn insert_spell_at_pos(
    trigger: Trigger<AddSpellTo>,
    mut wand_inventory: ResMut<SpellInventory>,
    mut player_caster: Query<&mut PlayerSpellTrigger>,
) {
    info!(
        "inserted spell: {} at pos: {}",
        trigger.event().0.data.get_name(),
        trigger.event().1.get_index(&wand_inventory.spells)
    );
    wand_inventory.insert_spell(trigger.event().0.clone(), trigger.event().1);
    wand_inventory.rebuild_effects();
    debug!("effects: {:?}", wand_inventory.spell_effects);

    player_caster.single_mut().spells = Arc::new(wand_inventory.spell_effects.clone());
}

fn rebuild_wand(
    _trigger: Trigger<RebuildWand>,
    mut wand_inventory: ResMut<SpellInventory>,
    mut player_caster: Query<&mut PlayerSpellTrigger>,
) {
    info!(
        "rebuilt wand with spells: {}",
        wand_inventory
            .spells
            .iter()
            .map(|s| s.data.get_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
    wand_inventory.rebuild_effects();
    info!("effects: {:?}", wand_inventory.spell_effects);
    player_caster.single_mut().spells = Arc::new(wand_inventory.spell_effects.clone());
}
