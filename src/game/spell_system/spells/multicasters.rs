use std::slice::Iter;
use std::sync::Arc;

use bevy::prelude::{Entity, World};
use log::warn;

use crate::game::spell_system::casting::{SpellCastContext, SpellCastValues};
use crate::game::spell_system::triggers::{do_collision_trigger, CollisionSpellTrigger};
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect, SpellModifier};

pub(super) fn get_spells() -> Vec<(SpellComponent, i32)> {
    vec![
        (
            SpellComponent {
                data: Box::new(ContactCasterData {
                    spells_triggered: 1,
                }),
                icon_id: 24,
            },
            10,
        ),
        (
            SpellComponent {
                data: Box::new(ScatterCastData {
                    spell_count: 2,
                    spread: 30.0,
                }),
                icon_id: 26,
            },
            10,
        ),
        (
            SpellComponent {
                data: Box::new(ScatterCastData {
                    spell_count: 3,
                    spread: 45.0,
                }),
                icon_id: 27,
            },
            25,
        ),
        (
            SpellComponent {
                data: Box::new(ScatterCastData {
                    spell_count: 4,
                    spread: 90.0,
                }),
                icon_id: 28,
            },
            80,
        ),
        (
            SpellComponent {
                data: Box::new(BurstCastData { spell_count: 2 }),
                icon_id: 29,
            },
            15,
        ),
        (
            SpellComponent {
                data: Box::new(BurstCastData { spell_count: 3 }),
                icon_id: 30,
            },
            30,
        ),
        (
            SpellComponent {
                data: Box::new(BurstCastData { spell_count: 4 }),
                icon_id: 28,
            },
            100,
        ),
    ]
}

#[derive(Clone)]
pub struct ContactCasterData {
    pub spells_triggered: usize,
}
impl SpellData for ContactCasterData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let trigger_spell = iter.next()?.data.build(iter)?;
        let mut spells_triggered: Vec<Arc<dyn SpellEffect>> = Vec::new();

        for _ in 0..self.spells_triggered {
            let Some(next) = iter.next() else {
                warn!("Failed to build trigger's child spell, not enough spells in the list.");
                break;
            }; //no more spell_system left to add to this trigger

            let Some(spell) = next.data.build(iter) else {
                warn!("failed to build trigger's child spell, failed to build child spell");
                break;
            }; //failed to build child spell

            spells_triggered.push(spell);
        }

        Some(Arc::new(ContactCaster {
            trigger_spell,
            spells_triggered: Arc::new(spells_triggered),
        }))
    }

    fn get_name(&self) -> String {
        "Collision Trigger".to_string()
    }

    fn get_desc(&self) -> String {
        "When the following spell's projectiles collide with something, they cast the immediately following spell.".to_string()
    }
}
#[derive(Debug, Clone)]
pub struct ContactCaster {
    pub trigger_spell: Arc<dyn SpellEffect>,
    pub spells_triggered: Arc<Vec<Arc<dyn SpellEffect>>>,
}
impl SpellEffect for ContactCaster {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let spells = self.spells_triggered.clone();
        let new_context = context.fresh_clone();
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            let mut spell_context = new_context.clone();
            spell_context.caster = e;
            mod_world.entity_mut(e).insert((CollisionSpellTrigger {
                values: SpellCastValues::default(),
                spells: spells.clone(),
            },));
            mod_world.entity_mut(e).observe(do_collision_trigger);
        });
        context.add_modifier("CollisionTrigger", modifier);
        self.trigger_spell.cast(context, world);
    }
}

#[derive(Clone)]
pub struct BurstCastData {
    pub spell_count: usize,
}
impl SpellData for BurstCastData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let mut spells: Vec<Arc<dyn SpellEffect>> = Vec::new();

        for _ in 0..self.spell_count {
            let Some(next) = iter.next() else {
                warn!("Failed to build burst's child spell, not enough spells in the list.");
                break;
            }; //no more spell_system left to add to this burst

            let Some(spell) = next.data.build(iter) else {
                warn!("failed to build burst's child spell, failed to build child spell");
                break;
            }; //failed to build child spell

            spells.push(spell);
        }

        Some(Arc::new(BurstCast {
            spells: Arc::new(spells),
        }))
    }

    fn get_name(&self) -> String {
        "Burst ".to_string() + &self.spell_count.to_string()
    }

    fn get_desc(&self) -> String {
        "Casts the following ".to_string()
            + &self.spell_count.to_string()
            + " spells at the same time."
    }
}

#[derive(Debug, Clone)]
pub struct BurstCast {
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
impl SpellEffect for BurstCast {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        for spell in self.spells.iter() {
            spell.cast(context, world);
        }
    }
}

#[derive(Clone)]
pub struct ScatterCastData {
    pub spell_count: usize,
    pub spread: f32,
}
impl SpellData for ScatterCastData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let mut spells: Vec<Arc<dyn SpellEffect>> = Vec::new();

        for _ in 0..self.spell_count {
            let Some(next) = iter.next() else {
                warn!("Failed to build burst's child spell, not enough spells in the list.");
                break;
            }; //no more spell_system left to add to this burst

            let Some(spell) = next.data.build(iter) else {
                warn!("failed to build burst's child spell, failed to build child spell");
                break;
            }; //failed to build child spell

            spells.push(spell);
        }

        Some(Arc::new(ScatterCast {
            spells: Arc::new(spells),
            spread: self.spread,
        }))
    }

    fn get_name(&self) -> String {
        "Scatter ".to_string() + &self.spell_count.to_string()
    }

    fn get_desc(&self) -> String {
        "Casts the following ".to_string()
            + &self.spell_count.to_string()
            + " spells, with a random spread."
    }
}

#[derive(Debug, Clone)]
pub struct ScatterCast {
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
    pub spread: f32,
}
impl SpellEffect for ScatterCast {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let mut cast_context = context.clone();
        cast_context.values.spread += self.spread;

        for spell in self.spells.iter() {
            spell.cast(&mut cast_context, world);
        }
    }
}
