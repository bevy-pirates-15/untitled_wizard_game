use std::f32::consts::PI;
/// Module for spell triggers.
///
/// Triggers are components that are added to entities to trigger spell_system.
/// They require some sort of SpellCaster to also be present on the entity
use std::sync::Arc;

use bevy::app::{App, Update};
use bevy::math::Quat;
use bevy::prelude::{
    in_state, BuildChildren, Commands, Component, Entity, GlobalTransform, IntoSystemConfigs,
    Query, Res, SpatialBundle, Timer, Transform, Trigger, With,
};
use bevy::time::Time;
use leafwing_input_manager::action_state::ActionState;

use crate::game::input::PlayerAction;
use crate::game::projectiles::ProjectileCollisionEvent;
use crate::game::spell_system::casting::{
    InstantCaster, SequentialCaster, SpellCastValues, SpellCaster,
};
use crate::game::spell_system::SpellEffect;
use crate::screen::GameState;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_timer_trigger
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Running)),
            (do_timer_trigger, do_player_trigger).in_set(AppSet::Update),
            //todo: more triggers
        ),
    );
}

#[derive(Component, Debug, Clone)]
pub struct PlayerSpellTrigger {
    pub current_caster: Option<Entity>,
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
pub fn do_player_trigger(
    action_state: Res<ActionState<PlayerAction>>,
    mut player_trig: Query<(Entity, &mut PlayerSpellTrigger)>,
    casters: Query<(), With<SpellCaster>>,
    mut commands: Commands,
) {
    if action_state.pressed(&PlayerAction::Shoot) {
        //spawn a spell caster on the trigger:
        for (e, mut trigger) in player_trig.iter_mut() {
            //check child still exists
            if let Some(caster) = trigger.current_caster {
                if casters.get(caster).is_ok() {
                    continue;
                } else {
                    trigger.current_caster = None;
                }
            }

            let caster = commands.spawn((
                SpellCaster::Sequential(SequentialCaster::new(
                    trigger.values.clone(),
                    trigger.spells.clone(),
                )),
                SpatialBundle::default(),
            ));

            let c_id = caster.id();

            commands.entity(e).add_child(c_id);
            trigger.current_caster = Some(c_id);
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
pub fn do_timer_trigger(
    mut timer_trigger: Query<(&GlobalTransform, &TimerSpellTrigger)>,
    mut commands: Commands,
) {
    for (transform, trigger) in timer_trigger.iter_mut() {
        if trigger.timer.just_finished() {
            commands.spawn((
                SpellCaster::Sequential(SequentialCaster::new(
                    trigger.values.clone(),
                    trigger.spells.clone(),
                )),
                SpatialBundle::from_transform(transform.compute_transform()),
            ));
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct CollisionSpellTrigger {
    pub values: SpellCastValues,
    pub spells: Arc<Vec<Arc<dyn SpellEffect>>>,
}
/// Function to trigger spell_system when a projectile collides with something.
/// has to be added via observers.
#[allow(dead_code)]
pub fn do_collision_trigger(
    trigger: Trigger<ProjectileCollisionEvent>,
    // mut collision_triggers: Query<(&CollisionSpellTrigger, &mut SpellCaster)>,
    mut collision_triggers: Query<(&GlobalTransform, &CollisionSpellTrigger)>,
    mut entity_hit: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let proj_entity = trigger.entity();
    let Ok((transform, col_trigger)) = collision_triggers.get_mut(proj_entity) else {
        return;
    };

    let Ok(other_transform) = entity_hit.get_mut(trigger.event().target) else {
        return;
    };

    //cast spell from other entity in vector between projectile and other entity
    let cast_vec = (other_transform.translation() - transform.translation()).normalize();
    let cast_transform = Transform::from_translation(other_transform.translation()).with_rotation(
        Quat::from_rotation_z(cast_vec.y.atan2(cast_vec.x) - PI / 2.),
    );
    commands.spawn((
        SpellCaster::Instant(InstantCaster::new(
            col_trigger.values.clone(),
            col_trigger.spells.clone(),
        )),
        SpatialBundle::from_transform(cast_transform),
    ));

    // do_caster(
    //     collision_triggers
    //         .transmute_lens::<(
    //             Entity,
    //             &mut SpellCaster,
    //             &CasterTargeter,
    //             Option<&DeleteOnCast>,
    //         )>()
    //         .query(),
    //     commands,
    // );
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
//                     to_trigger_spells: trigger.spell_system.clone(),
//                 };
//                 world.commands().trigger_targets(spell_cast_event,entity);
//             },
//         );
//     }
// }
