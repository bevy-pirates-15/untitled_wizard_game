/// Module for spell triggers.
///
/// Triggers are components that are added to entities to trigger spells.
/// They require some sort of SpellCaster to also be present on the entity
use std::sync::Arc;

use bevy::app::{App, Update};
use bevy::prelude::{Component, IntoSystemConfigs, Query, Res, Timer, Trigger};
use bevy::time::Time;
use leafwing_input_manager::action_state::ActionState;

use crate::game::input::PlayerAction;
use crate::game::projectiles::ProjectileCollisionEvent;
use crate::game::spells::casting::{SpellCastValues, SpellCaster};
use crate::game::spells::SpellEffect;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_timer_trigger.in_set(AppSet::TickTimers),
            (do_timer_trigger, do_player_trigger).in_set(AppSet::Update),
            //todo: more triggers
        ),
    );
}

#[derive(Component, Debug, Clone)]
pub struct PlayerSpellTrigger {
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
pub fn do_player_trigger(
    action_state: Res<ActionState<PlayerAction>>,
    mut player_trig: Query<(&mut SpellCaster, &PlayerSpellTrigger)>,
) {
    if action_state.pressed(&PlayerAction::Shoot) {
        for (mut caster, trigger) in player_trig.iter_mut() {
            caster.try_cast(trigger.values.clone(), trigger.spells.clone());
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TimerSpellTrigger {
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
    pub timer: Timer,
}
pub fn tick_timer_trigger(time: Res<Time>, mut timer_trigger: Query<&mut TimerSpellTrigger>) {
    for mut trigger in timer_trigger.iter_mut() {
        trigger.timer.tick(time.delta());
    }
}
pub fn do_timer_trigger(mut timer_trigger: Query<(&mut SpellCaster, &TimerSpellTrigger)>) {
    for (mut caster, trigger) in timer_trigger.iter_mut() {
        if trigger.timer.just_finished() {
            caster.try_cast(trigger.values.clone(), trigger.spells.clone());
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct CollisionSpellTrigger {
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
/// Function to trigger spells when a projectile collides with something.
/// has to be added via observers.
#[allow(dead_code)]
pub fn do_collision_trigger(
    trigger: Trigger<ProjectileCollisionEvent>,
    mut collision_triggers: Query<(&CollisionSpellTrigger, &mut SpellCaster)>,
) {
    let proj_entity = trigger.entity();
    let Ok((trigger, mut caster)) = collision_triggers.get_mut(proj_entity) else {
        return;
    };
    caster.try_cast(trigger.values.clone(), trigger.spells.clone());
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExpirationSpellTrigger {
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
// impl Component for ExpirationSpellTrigger {
//     const STORAGE_TYPE: StorageType = StorageType::Table;
//
//     fn register_component_hooks(_hooks: &mut ComponentHooks) {
//         _hooks.on_remove(
//             |mut world, entity, id| {
//                 let trigger = world.get_entity_mut(entity).unwrap().get::<ExpirationSpellTrigger>().unwrap();
//                 let spell_cast_event = SpellCastEvent {
//                     to_trigger_values: trigger.values.clone(),
//                     to_trigger_spells: trigger.spells.clone(),
//                 };
//                 world.commands().trigger_targets(spell_cast_event,entity);
//             },
//         );
//     }
// }
