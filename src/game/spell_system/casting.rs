use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::app::{App, Update};
use bevy::log::info;
use bevy::math::EulerRot;
use bevy::prelude::{
    in_state, Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform, IntoSystemConfigs,
    Query, Reflect, Res, Time, Timer, TimerMode, Vec2, World,
};

use crate::game::spell_system::{SpellEffect, SpellModifier, SpellModifierNode};
use crate::screen::GameState;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_sequential_caster
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Running)),
            do_caster.in_set(AppSet::Update),
        ),
    )
    .register_type::<SpellCaster>()
    .register_type::<SequentialCaster>()
    .register_type::<InstantCaster>();
}

/////////////////////
// CASTING CONTEXT //
/////////////////////

/// values passed down the spell chain, modified by modifiers, and used on spell cast
#[derive(Debug, Clone, Default)]
pub struct SpellCastValues {
    #[allow(dead_code)]
    pub spread: f32, //used for multicasting/multishot spell_system
    pub modifiers: Arc<SpellModifierNode>, //modifiers to apply to the spell
}

/// context used to cast a spell
#[derive(Debug, Clone)]
pub struct SpellCastContext {
    pub caster: Entity,
    pub direction: Vec2,
    inherit_direction: bool,
    //can be increased/decreases as spell_system are cast, and is accessed to add a delay post-cast
    pub spell_delay: Arc<Mutex<Duration>>,
    pub values: SpellCastValues,
}
impl SpellCastContext {
    pub fn add_modifier(&mut self, id: &str, modifier: SpellModifier) {
        self.values.modifiers =
            SpellModifierNode::with_new(id, self.values.modifiers.clone(), modifier);
    }

    pub fn fresh_clone(&self) -> Self {
        SpellCastContext {
            caster: self.caster,
            direction: self.direction,
            inherit_direction: self.inherit_direction,
            spell_delay: Arc::new(Mutex::new(*self.spell_delay.lock().unwrap())),
            values: self.values.clone(),
        }
    }
}

//////////////////
// SPELL CASTER //
//////////////////
#[derive(Reflect, Component, Debug, Clone)]
pub enum SpellCaster {
    Sequential(SequentialCaster),
    Instant(InstantCaster),
}
impl SpellCaster {
    pub fn get_next_casts(&mut self) -> (SpellCastValues, Vec<Arc<dyn SpellEffect>>) {
        match self {
            Self::Sequential(caster) => (caster.cast_values.clone(), caster.get_next_cast()),
            Self::Instant(caster) => (caster.cast_values.clone(), caster.get_next_cast()),
        }
    }
    fn get_base_spell_delay(&self) -> Duration {
        match self {
            Self::Sequential(caster) => caster.base_spell_delay,
            Self::Instant(_) => Duration::from_secs_f32(0.0),
        }
    }
    fn add_spell_delay(&mut self, delay: Duration) {
        match self {
            Self::Sequential(caster) => caster.add_spell_delay(delay),
            Self::Instant(_) => (),
        }
    }
    fn can_delete(&self) -> bool {
        match self {
            Self::Sequential(caster) => {
                caster.spell_queue.is_empty()
                    && caster.caster_delay.finished()
                    && caster.spell_delay.finished()
            }
            Self::Instant(caster) => caster.spell_list.is_empty(),
        }
    }
}

////////////////
// SEQ CASTER //
////////////////
/// instead of directly casting a spell through an observer, you can instead add a sequential caster to an entity
/// then instead of casting the spell directly, the observer will add the spell to the sequential caster's queue
/// this is used for the players wand, which has a mouse click trigger
#[derive(Debug, Clone, Reflect)]
pub struct SequentialCaster {
    #[reflect(ignore)]
    pub spell_queue: Vec<Arc<dyn SpellEffect>>,
    #[reflect(ignore)]
    pub cast_values: SpellCastValues,
    pub base_spell_delay: Duration,
    spell_delay: Timer,
    pub base_caster_delay: Duration,
    caster_delay: Timer,
}
impl SequentialCaster {
    pub fn new(cast_values: SpellCastValues, spells: Arc<Vec<Arc<dyn SpellEffect>>>) -> Self {
        let mut spell_queue = Vec::<Arc<dyn SpellEffect>>::new();
        for spell in spells.iter().rev() {
            spell_queue.push(spell.clone());
        }
        Self {
            spell_queue,
            cast_values,
            base_spell_delay: Duration::from_secs_f32(0.1),
            spell_delay: Timer::from_seconds(0.0, TimerMode::Once),
            base_caster_delay: Duration::from_secs_f32(0.5),
            caster_delay: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }

    fn get_next_cast(&mut self) -> Vec<Arc<dyn SpellEffect>> {
        //can cast?
        if !self.spell_delay.finished() || !self.caster_delay.finished() {
            return vec![];
        }

        let spell = self.spell_queue.pop();
        if self.spell_queue.is_empty() {
            self.caster_delay.set_duration(self.base_caster_delay);
            self.caster_delay.reset();
        }

        spell.map_or_else(Vec::new, |spell| vec![spell])
    }

    fn add_spell_delay(&mut self, delay: Duration) {
        self.spell_delay.set_duration(delay);
        self.spell_delay.reset();
    }
}
pub fn tick_sequential_caster(time: Res<Time>, mut caster: Query<&mut SpellCaster>) {
    for caster in caster.iter_mut() {
        if let SpellCaster::Sequential(ref mut sequential_caster) = caster.into_inner() {
            sequential_caster.spell_delay.tick(time.delta());
            sequential_caster.caster_delay.tick(time.delta());
        }
    }
}

////////////////////
// INSTANT CASTER //
////////////////////
#[derive(Debug, Clone, Reflect)]
pub struct InstantCaster {
    #[reflect(ignore)]
    pub spell_list: Vec<Arc<dyn SpellEffect>>,
    #[reflect(ignore)]
    pub cast_values: SpellCastValues,
}
impl InstantCaster {
    pub fn new(cast_values: SpellCastValues, spells: Arc<Vec<Arc<dyn SpellEffect>>>) -> Self {
        Self {
            spell_list: spells.to_vec(),
            cast_values,
        }
    }

    fn get_next_cast(&mut self) -> Vec<Arc<dyn SpellEffect>> {
        //take ALL the spells out the list, replace with empty list
        let spells = self.spell_list.clone();
        self.spell_list = vec![];

        //return the spells
        spells
    }
}

pub fn do_caster(
    mut q_caster: Query<(Entity, &mut SpellCaster, &GlobalTransform)>,
    mut commands: Commands,
) {
    for (ent, mut caster, g_transform) in q_caster.iter_mut() {
        //check if caster can be deleted first:
        if caster.can_delete() {
            if let Some(ecmd) = commands.get_entity(ent) {
                ecmd.despawn_recursive();
            }
            info!("removed spell caster");
        }

        let (values, spells) = caster.get_next_casts();

        if spells.is_empty() {
            continue;
        }

        let (z, _, _) = g_transform
            .compute_transform()
            .rotation
            .to_euler(EulerRot::ZXY);

        //build new context from castvalues
        let context = SpellCastContext {
            caster: ent,
            direction: Vec2::new(-z.sin(), z.cos()),
            inherit_direction: true,
            spell_delay: Arc::new(Mutex::new(caster.get_base_spell_delay())),
            values,
        };
        for spell in spells {
            let mut cast_context = context.clone();
            commands.add(move |w: &mut World| {
                spell.cast(&mut cast_context, w);
            });
        }
        let delay = *context.spell_delay.lock().unwrap();
        caster.add_spell_delay(delay);
        info!("delay: {}", delay.as_secs_f32());
    }
}
// pub fn update_rotation_based_targeter(
//     mut q_targeter: Query<(&mut CasterTargeter, &GlobalTransform)>,
// ) {
//     for (targeter, transform) in q_targeter.iter_mut() {
//         if let CasterTargeter::RotationBased(ref mut vec) = targeter.into_inner() {
//             let (z, _, _) = transform
//                 .compute_transform()
//                 .rotation
//                 .to_euler(EulerRot::ZXY);
//             *vec = Vec2::new(-z.sin(), z.cos());
//         }
//     }
// }

// pub fn update_random_targeter(mut q_targeter: Query<(&mut CasterTargeter)>) {
//     todo!();
// }
//
// pub fn update_nearest_hostile_targeter(
//     mut q_targeter: Query<(&mut CasterTargeter, &GlobalTransform)>,
// ) {
//     todo!();
// }
