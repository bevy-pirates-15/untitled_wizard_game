use bevy::prelude::*;

use crate::{config::LEVEL_EXP_LIST, screen::GameState};

use super::{spawn::player::Player, ItemDrop};

pub(super) fn plugin(app: &mut App) {
    app.observe(level_up);
    app.add_systems(Update, (collect_xp,).run_if(in_state(GameState::Running)));
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

fn collect_xp(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut PlayerLevel), With<Player>>,
    xp_drops_query: Query<(&Transform, &Experience, Entity), With<ItemDrop>>,
) {
    if player_query.is_empty() || xp_drops_query.is_empty() {
        return;
    }

    let (player_pos, mut player_level) = player_query.single_mut();

    for (xp_pos, xp_amount, xp) in xp_drops_query.iter() {
        if xp_pos == player_pos {
            // Player is 'on' the xp drop
            let extra_xp = xp_amount.0 - player_level.exp_to_level_up;
            if extra_xp >= 0. {
                // player_level.level += 1;
                player_level.exp_to_level_up = LEVEL_EXP_LIST[player_level.level];
                player_level.overflow += extra_xp;
                commands.trigger(LevelUp)
            } else {
                player_level.exp_to_level_up -= xp_amount.0;
            }
            commands.entity(xp).despawn();
        }
    }
}

#[derive(Debug, Event)]
struct LevelUp;

fn level_up(
    _trigger: Trigger<LevelUp>,
    mut commands: Commands,
    mut player_query: Query<&mut PlayerLevel, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut player = player_query.single_mut();

    player.level += 1;

    // todo do level up specifics here

    let extra_overflow = player.overflow - player.exp_to_level_up;
    if extra_overflow >= 0. {
        player.exp_to_level_up = LEVEL_EXP_LIST[player.level];
        player.overflow = extra_overflow;
        commands.trigger(LevelUp)
    } else {
        player.exp_to_level_up -= player.overflow;
    }
}
