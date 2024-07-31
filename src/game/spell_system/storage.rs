use std::cmp::Ordering;
use std::sync::Arc;

use crate::game::spawn::wand::SpawnWand;
use crate::game::spell_system::triggers::PlayerSpellTrigger;
use crate::game::spell_system::{SpellComponent, SpellEffect};
use bevy::app::App;
use bevy::prelude::{Commands, Event, Query, ResMut, Resource, Trigger};
use log::{debug, info};
use rand::Rng;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpellPool>()
        .init_resource::<SpellInventory>()
        .observe(insert_spell_at_pos)
        .observe(rebuild_wand)
        .observe(new_wand_spells);
}

#[derive(Resource, Default)]
pub struct SpellPool {
    pub spells: Vec<(SpellComponent, i32)>,
}
impl SpellPool {
    // pub fn get_random_spell_component(&self) -> &SpellComponent {
    //     //gets random spell based on the weights
    //     let total_weight = self.spells.iter().map(|(_, w)| w).sum::<i32>();
    //     let mut rng = rand::thread_rng();
    //     let mut roll = rng.gen_range(0..total_weight);
    //     for (spell, weight) in &self.spells {
    //         if roll < *weight {
    //             return spell;
    //         }
    //         roll -= *weight;
    //     }
    //     panic!("Failed to get random spell");
    // }

    pub fn get_x_random_unique_spell_components(&self, x: usize) -> Vec<&SpellComponent> {
        // gets x random unique spells, based on the weights
        let mut rng = rand::thread_rng();
        let spell_indexes = (0..self.spells.len()).collect::<Vec<_>>();

        let mut weighted_indexes = spell_indexes
            .iter()
            .map(|i| (*i, 1. - rng.gen::<f32>().powi(self.spells[*i].1)))
            .collect::<Vec<_>>();
        weighted_indexes.sort_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap_or(Ordering::Equal));

        weighted_indexes
            .iter()
            .take(x)
            .map(|(i, _)| &self.spells[*i].0)
            .collect()
    }

    pub(crate) fn insert_spells(&mut self, spells: Vec<(SpellComponent, i32)>) {
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

    for mut caster in player_caster.iter_mut() {
        caster.spells = Arc::new(wand_inventory.spell_effects.clone());
    }
}

pub fn new_wand_spells(
    _trigger: Trigger<SpawnWand>,
    mut inventory: ResMut<SpellInventory>,
    mut commands: Commands,
) {
    inventory.spells.clear();
    // inventory.push_spell(SpellComponent {
    //     data: Box::new(
    //         crate::game::spell_system::spells::modifiers::DuplicateData {
    //             spread_increase: 40.,
    //             bullet_count: 3,
    //             damage_decrease: 0.5,
    //         },
    //     ),
    //     icon_id: 18,
    // });
    inventory.push_spell(SpellComponent {
        data: Box::new(crate::game::spell_system::spells::cores::ZapSpellData {
            base_damage: 40.0,
        }),
        icon_id: 0,
    });
    // inventory.push_spell(SpellComponent {
    //     data: Box::new(crate::game::spell_system::spells::cores::BangSpellData {
    //         base_damage: 40.0,
    //         radius: 30.0,
    //     }),
    //     icon_id: 3,
    // },);
    // inventory.push_spell(SpellComponent {
    //     data: Box::new(crate::game::spell_system::spells::cores::ArcaneArrowSpellData {
    //         base_damage: 30.0,
    //         speed: 400.0,
    //         num_hits: 3,
    //     }),
    //     icon_id: 1,
    // });
    // inventory.push_spell(SpellComponent {
    //     data: Box::new(crate::game::spell_system::spells::cores::SplitterBoltsSpellData {
    //         base_damage: 20.0,
    //         projectile_count: 3,
    //     }),
    //     icon_id: 2,
    // });

    commands.trigger(RebuildWand);
}
