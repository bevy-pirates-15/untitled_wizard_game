use crate::game::spawn::player::Player;
use crate::game::spell_system::spells::enemy::EnemySpell;
use crate::game::spell_system::triggers::TimerSpellTrigger;
use crate::screen::{GameState, Screen};
use crate::AppSet;
use bevy::app::{App, Update};
use bevy::hierarchy::BuildChildren;
use bevy::math::{Quat, Vec2, Vec3, Vec3Swizzles};
use bevy::prelude::{
    in_state, Commands, Component, Entity, GlobalTransform, IntoSystemConfigs, Query,
    SpatialBundle, StateScoped, Transform, With,
};
use bevy::time::{Timer, TimerMode};
use std::sync::Arc;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        do_enemy_aim
            .in_set(AppSet::Update)
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Component, Debug, Default)]
pub struct EnemyWand;

#[derive(Component, Default)]
pub struct EnemyAim(pub Vec2);

pub fn add_enemy_aim(entity: Entity, commands: &mut Commands) {
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            EnemyWand,
            EnemyAim::default(),
            StateScoped(Screen::Playing),
            TimerSpellTrigger {
                spells: Arc::new(vec![Arc::new(EnemySpell)]),
                timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                values: Default::default(),
            },
        ));
    });
}

fn do_enemy_aim(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut aim_query: Query<(&mut Transform, &GlobalTransform, &mut EnemyAim)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (mut transform, gtransform, mut aim) in aim_query.iter_mut() {
        let direction = player_transform.translation() - gtransform.translation();
        aim.0 = direction.xy().normalize();

        if direction.length_squared() > 0.01 {
            let angle = -direction.x.atan2(direction.y);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
