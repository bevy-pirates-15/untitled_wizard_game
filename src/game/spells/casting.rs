use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy::app::{App, Update};
use bevy::prelude::{
    Commands, Component, Entity, Event, IntoSystemConfigs, Query, Reflect, ReflectComponent, Res,
    Time, Timer, TimerMode, World,
};
use leafwing_input_manager::action_state::ActionState;

use crate::AppSet;
use crate::game::input::PlayerAction;
use crate::game::spells::{SpellEffect, SpellModifier, SpellModifierNode};
use crate::game::spells::triggers::ToTrigger;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_sequential_caster.in_set(AppSet::TickTimers),
            (do_sequential_caster, do_player_trigger).in_set(AppSet::Update),
        ),
    )
    .register_type::<SequentialCaster>();
}

/////////////////////
// CASTING CONTEXT //
/////////////////////
/// variables passed down the spell chain, modified by modifiers, and used on spell cast
#[derive(Debug, Clone)]
pub struct SpellCastContext {
    pub caster: Entity,
    pub spell_delay: Arc<Mutex<Duration>>, //its important to use arc here as the spell delay is retrieved AFTER cast
    pub spread: f32,                       //used for multicasting/multishot spells
    pub modifiers: Arc<SpellModifierNode>,
}
impl SpellCastContext {
    pub fn add_modifier(&mut self, id: &str, modifier: SpellModifier) {
        self.modifiers = SpellModifierNode::with_new(id, self.modifiers.clone(), modifier);
    }

    pub fn fresh_clone(&self) -> Self {
        SpellCastContext {
            caster: self.caster,
            spell_delay: Arc::new(Mutex::new(*self.spell_delay.lock().unwrap())),
            spread: self.spread,
            modifiers: self.modifiers.clone(),
        }
    }
}

///////////////////////
// SEQUENTIAL CASTER //
///////////////////////
/// instead of directly casting a spell through an observer, you can instead add a sequential caster to an entity
/// then instead of casting the spell directly, the observer will add the spell to the sequential caster's queue
/// this is used for the players wand, which has a mouse click trigger
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SequentialCaster {
    #[reflect(ignore)]
    pub cast_data: Option<(SpellCastContext, Vec<Arc<dyn SpellEffect>>)>,
    pub base_spell_delay: Duration,
    spell_delay: Timer,
    pub base_cast_delay: Duration,
    cast_delay: Timer,
}
impl SequentialCaster {
    pub fn new() -> Self {
        Self {
            cast_data: None,
            base_spell_delay: Duration::from_secs_f32(0.1),
            spell_delay: Timer::from_seconds(0.0, TimerMode::Once),
            base_cast_delay: Duration::from_secs_f32(0.5),
            cast_delay: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
    pub fn try_cast(&mut self, to_trigger: ToTrigger) {
        if self.spell_delay.finished() && self.cast_delay.finished() && self.cast_data.is_none() {
            let mut spell_queue = Vec::<Arc<dyn SpellEffect>>::new();
            for spell in to_trigger.spells.iter().rev() {
                spell_queue.push(spell.clone());
            }

            self.cast_data = Some((to_trigger.context.clone(), spell_queue));
        }
    }
}
pub fn tick_sequential_caster(
    time: Res<Time>,
    mut sequential_trigger: Query<&mut SequentialCaster>,
) {
    for mut trigger in sequential_trigger.iter_mut() {
        trigger.spell_delay.tick(time.delta());
        trigger.cast_delay.tick(time.delta());
    }
}
pub fn do_sequential_caster(
    mut sequential_trigger: Query<&mut SequentialCaster>,
    mut commands: Commands,
) {
    for mut seq_caster in sequential_trigger.iter_mut() {
        //can cast?
        if !seq_caster.spell_delay.finished() {
            continue;
        }
        let Some(data) = &mut seq_caster.cast_data else {
            continue;
        };
        let Some(spell) = data.1.pop() else {
            seq_caster.cast_data = None;
            let delay = seq_caster.base_cast_delay;
            seq_caster.cast_delay.set_duration(delay);
            seq_caster.cast_delay.reset();
            continue;
        };

        let new_context = data.0.fresh_clone();
        let mut cast_context = new_context.clone();
        commands.add(move |w: &mut World| {
            spell.cast(&mut cast_context, w);
        });

        seq_caster
            .spell_delay
            .set_duration(*new_context.spell_delay.lock().unwrap());
        seq_caster.spell_delay.reset();
    }
}

#[derive(Event, Debug, Clone)]
pub struct SpellCastEvent {
    pub to_trigger: ToTrigger,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerSpellTrigger {
    pub to_trigger: ToTrigger,
}
pub fn do_player_trigger(
    action_state: Res<ActionState<PlayerAction>>,
    player_trig: Query<(Entity, &PlayerSpellTrigger)>,
    mut commands: Commands,
) {
    if action_state.pressed(&PlayerAction::Shoot) {
        println!("firing player trigger");
        for (entity, trigger) in player_trig.iter() {
            let to_trigger = trigger.to_trigger.clone();
            println!("firing spell");
            commands.trigger_targets(SpellCastEvent { to_trigger }, entity);
        }
    }
}
