//! Logic for dealing damage to the player

use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::{
    game::{audio::sfx::Sfx, enemy::Enemy, spawn::player::Player, Health},
    screen::GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (detect_enemy_player_collsion, handle_invincibility).run_if(in_state(GameState::Running)),
    );
}

#[derive(Component)]
struct Invincibility {
    timer: Timer,
}

impl Invincibility {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

// Detect enemy and player collide, take health away from player
fn detect_enemy_player_collsion(
    mut commands: Commands,
    mut death_state: ResMut<NextState<GameState>>,
    mut player_collision_query: Query<
        (
            &mut Health,
            Entity,
            &CollidingEntities,
            Option<&Invincibility>,
        ),
        (With<Player>, Without<Enemy>),
    >,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for (mut player_health, player_entity, colliding_entities, invincibility) in
        player_collision_query.iter_mut()
    {
        if invincibility.is_some() {
            continue;
        }
        // idk why clippy gets mad here, but i have to do this
        // otherwise it broken sadge
        #[allow(clippy::assign_op_pattern)]
        for &colliding_entity in colliding_entities.0.iter() {
            if enemy_query.contains(colliding_entity) {
                player_health.0 = player_health.0 - 1.0;
                println!("Player hit! Health: {:?}", player_health.0);
                if player_health.0 <= 0. {
                    death_state.set(GameState::Death);
                }
                commands.trigger(Sfx::WizardGetsHit);
                commands
                    .entity(player_entity)
                    .insert(Invincibility::new(5.0));
            }
            break;
        }
    }
}

fn handle_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invincibility)>,
) {
    for (entity, mut invincibility) in query.iter_mut() {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.finished() {
            commands.entity(entity).remove::<Invincibility>();
        }
    }
}
