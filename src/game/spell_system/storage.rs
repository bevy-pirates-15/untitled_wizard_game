use std::sync::Arc;

use bevy::app::{App, Startup};
use bevy::prelude::{Event, IntoSystemConfigs, Query, ResMut, Resource, Trigger};

use crate::game::spell_system::spells::load_spells;
use crate::game::spell_system::triggers::PlayerSpellTrigger;
use crate::game::spell_system::{SpellComponent, SpellEffect};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpellPool>()
        .init_resource::<SpellInventory>()
        //temp system to load 6 random spells into the spell inventory
        .add_systems(
            Startup,
            (|mut spell_inventory: ResMut<SpellInventory>, pool: ResMut<SpellPool>| {
                for _ in 0..6 {
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
    fn get_random_spell_component(&self) -> &SpellComponent {
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
    fn push_spell(&mut self, spell: SpellComponent) {
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
    fn insert_spell(&mut self, spell: SpellComponent, pos: SpellAddPos) {
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
    wand_inventory.insert_spell(trigger.event().0.clone(), trigger.event().1);
    wand_inventory.rebuild_effects();

    player_caster.single_mut().spells = Arc::new(wand_inventory.spell_effects.clone());
}

fn rebuild_wand(
    _trigger: Trigger<RebuildWand>,
    mut wand_inventory: ResMut<SpellInventory>,
    mut player_caster: Query<&mut PlayerSpellTrigger>,
) {
    wand_inventory.rebuild_effects();

    player_caster.single_mut().spells = Arc::new(wand_inventory.spell_effects.clone());
}
