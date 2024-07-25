use std::sync::{Arc, Mutex};
use std::time::Duration;

use avian2d::prelude::LinearVelocity;
use bevy::app::{App, Update};
use bevy::math::EulerRot;
use bevy::prelude::{
    in_state, Commands, Component, Entity, GlobalTransform, IntoSystemConfigs, Query, Res, Time,
    Timer, TimerMode, Vec2, World,
};

use crate::game::projectiles::ProjectileTeam;
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
            (
                update_rotation_based_targeter,
                update_velocity_based_targeter,
            )
                .in_set(AppSet::Update),
        ),
    );
}

/////////////////////
// CASTING CONTEXT //
/////////////////////

/// values passed down the spell chain, modified by modifiers, and used on spell cast
#[derive(Debug, Clone)]
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
#[derive(Component, Debug, Clone)]
pub enum SpellCaster {
    Sequential(SequentialCaster),
    Instant(InstantCaster),
}
impl SpellCaster {
    pub fn try_cast(&mut self, values: SpellCastValues, spells: Arc<Vec<Arc<dyn SpellEffect>>>) {
        match self {
            Self::Sequential(caster) => caster.try_cast(values, spells),
            Self::Instant(caster) => caster.try_cast(values, spells),
        }
    }
    pub fn get_next_casts(&mut self) -> Option<(SpellCastValues, Vec<Arc<dyn SpellEffect>>)> {
        match self {
            Self::Sequential(caster) => {
                Some((caster.cast_data.clone()?.0, vec![caster.get_next_cast()?]))
            }
            Self::Instant(caster) => caster
                .cast_data
                .take()
                .into_iter()
                .map(|(values, spells)| (values.clone(), spells.clone()))
                .next(),
        }
    }
    pub fn get_base_spell_delay(&self) -> Duration {
        match self {
            Self::Sequential(caster) => caster.base_spell_delay,
            Self::Instant(_) => Duration::from_secs_f32(0.0),
        }
    }
    pub fn add_spell_delay(&mut self, delay: Duration) {
        match self {
            Self::Sequential(caster) => caster.add_spell_delay(delay),
            Self::Instant(_) => (),
        }
    }
}

////////////////
// SEQ CASTER //
////////////////
/// instead of directly casting a spell through an observer, you can instead add a sequential caster to an entity
/// then instead of casting the spell directly, the observer will add the spell to the sequential caster's queue
/// this is used for the players wand, which has a mouse click trigger
#[derive(Debug, Clone)]
pub struct SequentialCaster {
    pub cast_data: Option<(SpellCastValues, Vec<Arc<dyn SpellEffect>>)>,
    pub base_spell_delay: Duration,
    spell_delay: Timer,
    pub base_caster_delay: Duration,
    caster_delay: Timer,
}
impl SequentialCaster {
    pub fn new() -> Self {
        Self {
            cast_data: None,
            base_spell_delay: Duration::from_secs_f32(0.1),
            spell_delay: Timer::from_seconds(0.0, TimerMode::Once),
            base_caster_delay: Duration::from_secs_f32(0.5),
            caster_delay: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }

    fn try_cast(&mut self, values: SpellCastValues, spells: Arc<Vec<Arc<dyn SpellEffect>>>) {
        if self.spell_delay.finished() && self.caster_delay.finished() && self.cast_data.is_none() {
            let mut spell_queue = Vec::<Arc<dyn SpellEffect>>::new();
            for spell in spells.iter().rev() {
                spell_queue.push(spell.clone());
            }

            self.cast_data = Some((values.clone(), spell_queue));
        }
    }

    fn get_next_cast(&mut self) -> Option<Arc<dyn SpellEffect>> {
        //can cast?
        if !self.spell_delay.finished() || !self.caster_delay.finished() {
            return None;
        }
        let Some(data) = &mut self.cast_data else {
            return None;
        };

        let spell = data.1.pop();
        if data.1.is_empty() {
            self.cast_data = None;
            let delay = self.base_caster_delay;
            self.caster_delay.set_duration(delay);
            self.caster_delay.reset();
        }

        spell
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
#[derive(Debug, Clone)]
pub struct InstantCaster {
    pub cast_data: Option<(SpellCastValues, Vec<Arc<dyn SpellEffect>>)>,
}
impl InstantCaster {
    pub fn new() -> Self {
        Self { cast_data: None }
    }

    fn try_cast(&mut self, values: SpellCastValues, spells: Arc<Vec<Arc<dyn SpellEffect>>>) {
        let mut spell_queue = Vec::<Arc<dyn SpellEffect>>::new();
        for spell in spells.iter() {
            spell_queue.push(spell.clone());
        }

        self.cast_data = Some((values.clone(), spell_queue));
    }
}

pub fn do_caster(
    mut q_caster: Query<(
        Entity,
        &mut SpellCaster,
        &CasterTargeter,
        Option<&DeleteOnCast>,
    )>,
    mut commands: Commands,
) {
    for (ent, mut caster, targeter, delete) in q_caster.iter_mut() {
        let Some((values, spells)) = caster.get_next_casts() else {
            continue;
        };

        //build new context from castvalues
        let context = SpellCastContext {
            caster: ent,
            direction: targeter.get_spell_vec(),
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
        caster.add_spell_delay(*context.spell_delay.lock().unwrap());

        if delete.is_some() {
            commands.entity(ent).despawn();
        }
    }
}

#[derive(Component, Debug, Clone)]
pub enum CasterTargeter {
    VelocityBased(Vec2),
    RotationBased(Vec2),
    #[allow(dead_code)]
    Random(Vec2),
    #[allow(dead_code)]
    NearestHostile(Vec2, ProjectileTeam),
}
impl CasterTargeter {
    pub fn get_spell_vec(&self) -> Vec2 {
        match self {
            Self::VelocityBased(vec) => *vec,
            Self::RotationBased(vec) => *vec,
            Self::Random(vec) => *vec,
            Self::NearestHostile(vec, _) => *vec,
        }
    }
}

pub fn update_velocity_based_targeter(
    mut q_targeter: Query<(&mut CasterTargeter, &LinearVelocity)>,
) {
    for (targeter, velocity) in q_targeter.iter_mut() {
        if let CasterTargeter::VelocityBased(ref mut vec) = targeter.into_inner() {
            *vec = Vec2::new(velocity.0.x, velocity.0.y).normalize();
        }
    }
}

pub fn update_rotation_based_targeter(
    mut q_targeter: Query<(&mut CasterTargeter, &GlobalTransform)>,
) {
    for (targeter, transform) in q_targeter.iter_mut() {
        if let CasterTargeter::RotationBased(ref mut vec) = targeter.into_inner() {
            let (z, _, _) = transform
                .compute_transform()
                .rotation
                .to_euler(EulerRot::ZXY);
            *vec = Vec2::new(-z.sin(), z.cos());
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct DeleteOnCast;

// pub fn update_random_targeter(mut q_targeter: Query<(&mut CasterTargeter)>) {
//     todo!();
// }
//
// pub fn update_nearest_hostile_targeter(
//     mut q_targeter: Query<(&mut CasterTargeter, &GlobalTransform)>,
// ) {
//     todo!();
// }
