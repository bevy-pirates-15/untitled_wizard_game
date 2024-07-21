use std::slice::Iter;
use std::sync::Arc;

use bevy::prelude::{
    Commands, Entity, GlobalTransform, Reflect, SpatialBundle, Timer, TimerMode, Trigger, World,
};

use crate::game::spells::{SpellComponent, SpellData, SpellEffect, SpellModifier};
use crate::game::spells::casting::SpellCastContext;
use crate::game::spells::triggers::{SpellTriggerEvent, TimerSpellTrigger, ToTrigger};

/////////////////////////////
// EXAMPLE IMPLEMENTATIONS //
/////////////////////////////
pub struct ZapSpellData {
    pub base_damage: f64,
}
impl SpellData for ZapSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        return Some(Arc::new(ZapSpell {
            base_damage: self.base_damage,
        }));
    }

    // fn build(&self) -> Arc<dyn SpellEffect> {
    //     Arc::new(ZapSpell {
    //         base_damage: self.base_damage,
    //     })
    // }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ZapSpell {
    pub base_damage: f64,
}
impl SpellEffect for ZapSpell {
    fn get_name(&self) -> &str {
        "Zap"
    }

    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(caster_gt) = world.entity(context.caster).get::<GlobalTransform>() else {
            println!("Tried to cast spell from an entity with no global transform");
            return;
        };
        let spell_transform = caster_gt.compute_transform();

        //create new spell entity:
        let spell = world.spawn(SpatialBundle {
            transform: spell_transform,
            ..Default::default()
        });

        //add relevant components to spell:
        // basic projectile setup here
        // spell.insert((ProjectileDamage { damage: self.base_damage },));
        //todo

        let spell_entity = spell.id();
        context.modifiers.apply(spell_entity, world);
        println!("Cast Zap - DMG: {}", self.base_damage); //todo - get damage from spell damage component
    }
}

pub struct TriggerSpellData {
    pub spells_triggered: usize,
}
impl SpellData for TriggerSpellData {
    fn build(&self, mut iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let trigger_spell = iter.next()?.data.build(&mut iter)?;
        let mut spells_triggered: Vec<Arc<dyn SpellEffect>> = Vec::new();

        for _ in 0..self.spells_triggered {
            let Some(next) = iter.next() else {
                break;
            }; //no more spells left to add to this trigger

            let Some(spell) = next.data.build(&mut iter) else {
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
        let to_trigger = self.spells_triggered.clone();
        let new_context = context.fresh_clone();
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            mod_world
                .entity_mut(e)
                .insert(TimerSpellTrigger {
                    to_trigger: ToTrigger::new(to_trigger.clone(), new_context.clone()),
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                })
                .observe(
                    |trigger: Trigger<SpellTriggerEvent>, mut commands: Commands| {
                        let spells = trigger.event().to_trigger.spells.clone();
                        let context = trigger.event().to_trigger.context.clone();
                        commands.add(move |w: &mut World| {
                            for spell in spells.iter() {
                                println!("Triggering spell: {}", spell.get_name());
                                spell.cast(&mut context.fresh_clone(), w);
                            }
                        });
                    },
                );
        });

        //when we multicast, we need to clone the context instead of passing it down,
        // this is because each spell in the multicast chain needs to have its own context

        context.add_modifier("TEST-TriggerMod", modifier);
        println!(
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
