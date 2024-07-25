use std::slice::Iter;
use std::sync::Arc;
use std::time::Duration;

use bevy::log::{info, warn};
use bevy::math::Vec2;
use bevy::prelude::{Entity, Reflect, Timer, TimerMode, World};

use crate::game::projectiles::ProjectileDamage;
use crate::game::spells::casting::{CasterTargeter, InstantCaster, SpellCastContext, SpellCaster};
use crate::game::spells::helpers::spawn_spell_projectile;
use crate::game::spells::triggers::TimerSpellTrigger;
use crate::game::spells::{SpellComponent, SpellData, SpellEffect, SpellModifier};

/////////////////////////////
// EXAMPLE IMPLEMENTATIONS //
/////////////////////////////
pub struct ZapSpellData {
    pub base_damage: f32,
}
impl SpellData for ZapSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(ZapSpell {
            base_damage: self.base_damage,
        }))
    }

    // fn build(&self) -> Arc<dyn SpellEffect> {
    //     Arc::new(ZapSpell {
    //         base_damage: self.base_damage,
    //     })
    // }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ZapSpell {
    pub base_damage: f32,
}
impl SpellEffect for ZapSpell {
    fn get_name(&self) -> &str {
        "Zap"
    }

    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(spell_entity) = spawn_spell_projectile(
            context,
            world,
            10.,
            500.0,
            self.base_damage,
            1,
            Duration::from_secs_f32(2.0),
        ) else {
            warn!("Failed to spawn zap spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Zap - DMG: {}", spell_damage);
    }
}

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
            }; //no more spells left to add to this trigger

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
}

// pub struct CastSpellsCommand(Arc<Vec<Arc<dyn SpellEffect>>>);
// impl Command for TriggerSpell {
//     fn apply(self, world: &mut World) {
//
//         // do whatever you want with `world` and `self.data` here
//     }
// }

// pub type SpellModifier = Box<dyn Fn(Entity,&mut World) + Send + Sync>;
#[derive(Debug, Clone)]
pub struct TriggerSpell {
    pub trigger_spell: Arc<dyn SpellEffect>,
    pub spells_triggered: Arc<Vec<Arc<dyn SpellEffect>>>,
}
impl SpellEffect for TriggerSpell {
    fn get_name(&self) -> &str {
        "Test Trigger Spell"
    }

    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let spells = self.spells_triggered.clone();
        let new_context = context.fresh_clone();
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            let mut spell_context = new_context.clone();
            spell_context.caster = e;
            mod_world.entity_mut(e).insert((
                SpellCaster::Instant(InstantCaster::new()),
                TimerSpellTrigger {
                    values: spell_context.values.clone(),
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    spells: spells.clone(),
                },
                CasterTargeter::VelocityBased(Vec2::new(0.0, 1.0)),
            ));
        });

        //when we multicast, we need to clone the context instead of passing it down,
        // this is because each spell in the multicast chain needs to have its own context

        context.add_modifier("TEST-TriggerMod", modifier);
        info!(
            "Added Trigger to {} - triggering: {}",
            self.trigger_spell.get_name(),
            self.spells_triggered
                .iter()
                .map(|s| s.get_name())
                .collect::<Vec<&str>>()
                .join(", ")
        );
        self.trigger_spell.cast(context, world);
    }
}

// pub struct CastTriggeredSpells {
//     trigger: Trigger<SpellTriggerEvent>,
//
//
// }

// pub struct ExampleModifierEffect {
//     pub modified_spell: Arc<dyn SpellEffect>,
// }
// impl SpellEffect for ExampleModifierEffect {
//     fn get_name(&self) -> &str {
//         todo!()
//     }
//
//     fn cast(&self, caster: Entity, world: &mut World) -> SpellSet {
//         let spell_set = self.modified_spell.cast(caster, world);
//
//         //todo: use world scope to get/add components on the spells in the spellset
//         //e.g. get damage components and add +5
//
//         return spell_set;
//     }
// }
//
// pub struct MulticastSpellsEffect {
//     pub spells: Vec<Arc<dyn SpellEffect>>,
// }
// impl SpellEffect for MulticastSpellsEffect {
//     fn get_name(&self) -> &str {
//         todo!()
//     }
//
//     fn cast(&self, caster: Entity, world: &mut World) -> SpellSet {
//         let mut spell_set : Vec<SpellSet> = Vec::new();
//         for spell in &self.spells {
//             spell_set.push(spell.cast(caster, world));
//         }
//         return SpellSet::Set(spell_set);
//     }
// }
