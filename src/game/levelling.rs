use avian2d::collision::CollidingEntities;
use bevy::prelude::*;

use crate::screen::GameState;

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
    level: u32,
    exp_to_level_up: u32,
    overflow: u32,
}

impl Default for PlayerLevel {
    fn default() -> Self {
        PlayerLevel {
            level: 1,
            exp_to_level_up: 100,
            overflow: 0,
        }
    }
}

#[derive(Debug, Component, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct Experience(pub u32);

fn detect_player_experience_collision(
    mut commands: Commands,
    mut player_collision_query: Query<(&mut PlayerLevel, &CollidingEntities), With<Player>>,
    exp_query: Query<(Entity, &Experience), With<Experience>>,
) {
    for (mut player_level, colliding_entities) in player_collision_query.iter_mut() {
        for &colliding_entity in colliding_entities.0.iter() {
            if let Ok((exp_entity, experience)) = exp_query.get(colliding_entity) {
                info!(
                    "Exp collected: {:?}, Exp until next level: {:?}",
                    experience.0, player_level.exp_to_level_up
                );
                if experience.0 >= player_level.exp_to_level_up {
                    player_level.overflow += experience.0 - player_level.exp_to_level_up;
                    player_level.exp_to_level_up = compute_next_level(player_level.level);
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

    if player.overflow >= player.exp_to_level_up {
        player.exp_to_level_up = compute_next_level(player.level);
        player.overflow -= player.exp_to_level_up;
        commands.trigger(LevelUp)
    } else {
        player.exp_to_level_up -= player.overflow;
    }
    next_game_state.set(GameState::GemSelection);
}

fn compute_next_level(curr_level: u32) -> u32 {
    match curr_level {
        n @ 1..=10 => 100 + (10 * n),
        n @ 11..=u32::MAX => 200 + (50 * n),
        _ => unreachable!("Level too small"),
    }
}
