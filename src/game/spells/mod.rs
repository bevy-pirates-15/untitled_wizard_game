mod triggers;

use crate::game::spells::triggers::TimerSpellTrigger;
use bevy::ecs::world::Command;
use bevy::prelude::{
    Commands, Entity, Event, GlobalTransform, In, Reflect, SpatialBundle, System, Timer, Trigger,
    World,
};
use bevy::time::TimerMode;
use std::fmt::Debug;
use std::ops::Deref;
use std::slice::Iter;
use std::sync::Arc;

#[derive(Clone)]
pub struct SpellComponent {
    data: Arc<dyn SpellData>,
    // pub icon: String, //todo
    // pub tier: u32, //todo
}

pub trait SpellData {
    //use arc as spell effects can be easily cloned into children,
    //and they persist when the main spell is modified
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>>;
    // fn get_desc(&self) -> String; //todo, build description in the data so it can use the numbers

    // fn can_upgrade(&self) -> bool; //todo
    // fn upgrade(&self); //todo
}

pub trait SpellEffect: Send + Sync + Debug {
    fn get_name(&self) -> &str;
    fn cast(&self, context: SpellCastContext, world: &mut World);
}

#[derive(Event, Debug, Clone)]
pub struct SpellTriggerEvent {
    pub to_trigger: Arc<Vec<Arc<dyn SpellEffect>>>,
}
impl SpellTriggerEvent {
    pub fn new(to_trigger: Arc<Vec<Arc<dyn SpellEffect>>>) -> Self {
        SpellTriggerEvent { to_trigger }
    }
}

pub type SpellModifier = Box<dyn Fn(Entity, &mut World)>;
pub struct SpellModifierNode {
    modifier: SpellModifier,
    prev: Option<Arc<SpellModifierNode>>,
}
impl SpellModifierNode {
    fn with_new(modifier: Arc<SpellModifierNode>, new_modifier: SpellModifier) -> Arc<Self> {
        Arc::new(Self {
            modifier: new_modifier,
            prev: Some(modifier),
        })
    }

    fn apply(&self, entity: Entity, world: &mut World) {
        (self.modifier)(entity, world);
        if let Some(ref prev) = &self.prev {
            prev.apply(entity, world);
        }
    }
}

// variables passed down the spell chain, modified by modifiers, and used on spell cast
pub struct SpellCastContext {
    pub caster: Entity,
    pub spell_delay: f32,
    pub spread: f32, //used for multicasting/multishot spells
    pub modifiers: Arc<SpellModifierNode>,
}
impl SpellCastContext {
    pub fn with_new_modifier(&self, modifier: SpellModifier) -> Self {
        SpellCastContext {
            caster: self.caster,
            spell_delay: self.spell_delay,
            spread: self.spread,
            modifiers: SpellModifierNode::with_new(self.modifiers.clone(), modifier),
        }
    }
}

/////////////////////////////
// EXAMPLE IMPLEMENTATIONS //
/////////////////////////////
pub struct ZapSpellData {
    pub base_damage: f64,
}
impl SpellData for ZapSpellData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
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

    fn cast(&self, context: SpellCastContext, world: &mut World) {
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
        println!("Cast Zap");

        //add relevant components to spell:
        // basic projectile setup here
        // spell.insert((ProjectileDamage { damage: self.base_damage },));
        //todo

        let spell_entity = spell.id();

        context.modifiers.apply(spell_entity, world);
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

    fn cast(&self, context: SpellCastContext, world: &mut World) {
        let to_trigger = self.spells_triggered.clone();
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            mod_world
                .entity_mut(e)
                .insert(TimerSpellTrigger {
                    to_trigger: to_trigger.clone(),
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                })
                .observe(|trigger: Trigger<SpellTriggerEvent>| {
                    for spell in trigger.event().to_trigger.iter() {
                        println!("Triggering spell: {}", spell.get_name());
                    }
                });
        });

        let new_context = context.with_new_modifier(modifier);
        self.trigger_spell.cast(new_context, world);
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
