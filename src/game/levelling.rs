use avian2d::collision::CollidingEntities;
use bevy::prelude::*;

use crate::{config::LEVEL_EXP_LIST, screen::GameState};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.observe(level_up);
    app.add_systems(
        Update,
        (detect_player_experience_collision).run_if(in_state(GameState::Running)),
    );
    app.register_type::<Experience>();
    app.register_type::<PlayerLevel>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerLevel {
    level: usize, // usize is far too big but otherwise we cant index the EXP_LIST
    exp_to_level_up: f64,
    overflow: f64,
}

impl Default for PlayerLevel {
    fn default() -> Self {
        PlayerLevel {
            level: 1,
            exp_to_level_up: LEVEL_EXP_LIST[0],
            overflow: 0.,
        }
    }
}

#[derive(Debug, Component, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct Experience(pub f64);

fn detect_player_experience_collision(
    mut commands: Commands,
    mut player_collision_query: Query<(&mut PlayerLevel, &CollidingEntities), With<Player>>,
    exp_query: Query<(Entity, &Experience), With<Experience>>,
) {
    for (mut player_level, colliding_entities) in player_collision_query.iter_mut() {
        for &colliding_entity in colliding_entities.0.iter() {
            if let Ok((exp_entity, experience)) = exp_query.get(colliding_entity) {
                let extra_exp = experience.0 - player_level.exp_to_level_up;
                info!(
                    "Exp collected: {:?}, Exp until next level: {:?}",
                    exp_entity, player_level.exp_to_level_up
                );
                if extra_exp >= 0. {
                    player_level.exp_to_level_up = LEVEL_EXP_LIST[player_level.level];
                    player_level.overflow += extra_exp;
                    commands.trigger(LevelUp)
                } else {
                    player_level.exp_to_level_up -= experience.0;
                }
                commands.entity(exp_entity).despawn();
            }
        }
    }
}

#[derive(Debug, Event)]
struct LevelUp;

fn level_up(
    _trigger: Trigger<LevelUp>,
    mut commands: Commands,
    mut player_query: Query<&mut PlayerLevel, With<Player>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player = player_query.single_mut();

    player.level += 1;
    info!("Player levels up to level {}", player.level);
    // todo do level up specifics here

    let extra_overflow = player.overflow - player.exp_to_level_up;
    if extra_overflow >= 0. {
        player.exp_to_level_up = LEVEL_EXP_LIST[player.level];
        player.overflow = extra_overflow;
        commands.trigger(LevelUp)
    } else {
        player.exp_to_level_up -= player.overflow;
    }
    next_game_state.set(GameState::GemSelection);
}
