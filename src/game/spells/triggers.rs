// triggers for casting attached spells

// player trigger (used for attaching to players wand)
// repeat trigger (casts its spell repeatedly, X amount of times)
// timer trigger (casts its spell after a certain delay)
// collision trigger (casts its spell when it collides with something)
// expiration trigger (casts its spell before it expires)

use crate::game::input::PlayerAction;
use crate::game::spells::{SpellEffect, SpellTriggerEvent};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Timer};
use bevy::time::Time;
use leafwing_input_manager::action_state::ActionState;
use std::sync::Arc;

pub type TriggeredSpells = Arc<Vec<Arc<dyn SpellEffect>>>;

#[derive(Component, Debug, Clone)]
pub struct PlayerSpellTrigger {
    pub to_trigger: TriggeredSpells,
}
pub fn do_player_trigger(
    action_state: Res<ActionState<PlayerAction>>,
    player_trig: Query<(Entity, &PlayerSpellTrigger)>,
    mut commands: Commands,
) {
    if action_state.pressed(&PlayerAction::Shoot) {
        for (entity, trigger) in player_trig.iter() {
            commands.trigger_targets(SpellTriggerEvent::new(trigger.to_trigger.clone()), entity);
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct RepeatSpellTrigger {
    pub to_trigger: TriggeredSpells,
    pub counter: Option<u32>,
}
pub fn do_repeat_trigger(
    repeat_trigger: Query<(Entity, &RepeatSpellTrigger)>,
    mut commands: Commands,
) {
    for (entity, trigger) in repeat_trigger.iter() {
        if let Some(mut counter) = trigger.counter {
            if counter <= 0 {
                continue;
            }
            counter -= 1;
        }
        commands.trigger_targets(SpellTriggerEvent::new(trigger.to_trigger.clone()), entity)
    }
}

#[derive(Component, Debug, Clone)]
pub struct TimerSpellTrigger {
    pub to_trigger: TriggeredSpells,
    pub timer: Timer,
}
pub fn tick_timer_trigger(time: Res<Time>, mut timer_trigger: Query<&mut TimerSpellTrigger>) {
    for mut trigger in timer_trigger.iter_mut() {
        trigger.timer.tick(time.delta());
    }
}
pub fn do_timer_trigger(
    timer_trigger: Query<(Entity, &TimerSpellTrigger)>,
    mut commands: Commands,
) {
    for (entity, mut trigger) in timer_trigger.iter() {
        if trigger.timer.finished() {
            commands.trigger_targets(SpellTriggerEvent::new(trigger.to_trigger.clone()), entity)
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CollisionSpellTrigger; // todo

#[derive(Component, Debug, Clone, PartialEq)]
pub struct ExpirationSpellTrigger; // todo
