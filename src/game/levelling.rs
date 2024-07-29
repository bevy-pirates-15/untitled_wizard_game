use avian2d::collision::CollidingEntities;
use bevy::prelude::*;

use crate::{
    config::{EXPERIENCE_RADIUS, EXPERIENCE_SPEED},
    game::audio::sfx::Sfx,
    screen::GameState,
};

use super::{enemy::Enemy, spawn::player::Player};

pub(super) fn plugin(app: &mut App) {
    app.observe(level_up);
    app.add_systems(
        Update,
        (
            detect_player_experience_collision,
            move_experience_towards_player,
        )
            .run_if(in_state(GameState::Running)),
    );
    app.register_type::<Experience>();
    app.register_type::<PlayerLevel>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerLevel {
    pub level: u32,
    pub exp_to_level_up: u32,
    overflow: u32,
}

#[derive(Component)]
pub struct LevelText;

impl Default for PlayerLevel {
    fn default() -> Self {
        PlayerLevel {
            level: 1,
            exp_to_level_up: compute_next_level(1),
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
    exp_query: Query<(Entity, &Experience), (With<Experience>, Without<Enemy>)>,
) {
    for (mut player_level, colliding_entities) in player_collision_query.iter_mut() {
        for &colliding_entity in colliding_entities.0.iter() {
            if let Ok((exp_entity, experience)) = exp_query.get(colliding_entity) {
                info!(
                    "Exp collected: {:?}, Exp until next level: {:?}",
                    experience.0, player_level.exp_to_level_up
                );
                commands.trigger(Sfx::PickUpExperience);
                if experience.0 >= player_level.exp_to_level_up {
                    player_level.overflow += experience.0 - player_level.exp_to_level_up;
                    player_level.exp_to_level_up = compute_next_level(player_level.level);
                    commands.trigger(LevelUp)
                } else {
                    player_level.exp_to_level_up -= experience.0;
                }
                commands.entity(exp_entity).despawn_recursive();
            }
        }
    }
}

fn move_experience_towards_player(
    time: Res<Time>,
    mut experience_query: Query<&mut Transform, (With<Experience>, Without<Enemy>)>,
    player_query: Query<&Transform, (With<Player>, Without<Experience>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut experience_transform in experience_query.iter_mut() {
            let distance = experience_transform
                .translation
                .distance(player_transform.translation);

            if distance < EXPERIENCE_RADIUS {
                let direction =
                    (player_transform.translation - experience_transform.translation).normalize();
                experience_transform.translation +=
                    direction * EXPERIENCE_SPEED * time.delta_seconds();
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
    mut level_text_query: Query<&mut Text, With<LevelText>>,
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
    commands.trigger(Sfx::LevelUp);
    let mut text = level_text_query.single_mut();
    text.sections[0].value = format!("Level {:?}", player.level);
    next_game_state.set(GameState::GemSelection);
}

pub fn compute_next_level(curr_level: u32) -> u32 {
    match curr_level {
        n @ 1..=10 => 100 + (10 * n),
        n @ 11..=u32::MAX => 200 + (50 * n),
        _ => unreachable!("Level too small"),
    }
}
