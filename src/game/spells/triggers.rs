use std::sync::Arc;

use bevy::app::{App, Update};
use bevy::prelude::{Commands, Component, Entity, Event, IntoSystemConfigs, Query, Res, Timer};
use bevy::time::Time;

use crate::game::spells::casting::SpellCastContext;
use crate::game::spells::SpellEffect;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_timer_trigger.in_set(AppSet::TickTimers),
            do_timer_trigger,
            //todo: more triggers
        ),
    );
}

#[derive(Clone, Debug)]
pub struct ToTrigger {
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
    pub context: SpellCastContext,
}
impl ToTrigger {
    pub fn new(spells: Arc<Vec<Arc<dyn SpellEffect>>>, context: SpellCastContext) -> Self {
        ToTrigger { spells, context }
    }
}

#[derive(Event, Debug, Clone)]
pub struct SpellTriggerEvent {
    pub to_trigger: ToTrigger,
}

#[derive(Component, Debug, Clone)]
pub struct TimerSpellTrigger {
    pub to_trigger: ToTrigger,
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
    for (entity, trigger) in timer_trigger.iter() {
        if trigger.timer.just_finished() {
            let to_trigger = trigger.to_trigger.clone();
            commands.trigger_targets(SpellTriggerEvent { to_trigger }, entity);
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CollisionSpellTrigger; // todo

#[derive(Component, Debug, Clone, PartialEq)]
pub struct ExpirationSpellTrigger; // todo
