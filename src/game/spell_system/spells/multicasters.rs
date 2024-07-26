use std::slice::Iter;
use std::sync::Arc;

use bevy::math::Vec2;
use bevy::prelude::{Entity, World};

use crate::game::spell_system::casting::{
    CasterTargeter, InstantCaster, SpellCastContext, SpellCaster,
};
use crate::game::spell_system::triggers::{do_collision_trigger, CollisionSpellTrigger};
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect, SpellModifier};

pub(super) fn get_spells() -> Vec<SpellComponent> {
    vec![SpellComponent {
        data: Box::new(TriggerSpellData {
            spells_triggered: 0,
        }),
        icon_id: 24,
    }]
}

#[derive(Clone)]
pub struct TriggerSpellData {
    pub spells_triggered: usize,
}
impl SpellData for TriggerSpellData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let trigger_spell = iter.next()?.data.build(iter)?;
        let mut spells_triggered: Vec<Arc<dyn SpellEffect>> = Vec::new();

        for _ in 0..self.spells_triggered {
            let Some(next) = iter.next() else {
                break;
            }; //no more spell_system left to add to this trigger

            let Some(spell) = next.data.build(iter) else {
                break;
            }; //failed to build child spell

            spells_triggered.push(spell);
        }

        Some(Arc::new(TriggerSpell {
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
pub struct TriggerSpell {
    pub trigger_spell: Arc<dyn SpellEffect>,
    pub spells_triggered: Arc<Vec<Arc<dyn SpellEffect>>>,
}
impl SpellEffect for TriggerSpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let spells = self.spells_triggered.clone();
        let new_context = context.fresh_clone();
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            let mut spell_context = new_context.clone();
            spell_context.caster = e;
            mod_world.entity_mut(e).insert((
                SpellCaster::Instant(InstantCaster::new()),
                CollisionSpellTrigger {
                    values: spell_context.values.clone(),
                    spells: spells.clone(),
                },
                CasterTargeter::VelocityBased(Vec2::new(0.0, 1.0)),
            ));
            mod_world.entity_mut(e).observe(do_collision_trigger);
        });
        context.add_modifier("CollisionTrigger", modifier);
        self.trigger_spell.cast(context, world);
    }
}
