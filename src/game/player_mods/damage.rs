//! Logic for dealing damage to the player

use crate::game::physics::GameLayer;
use crate::game::projectiles::HitByProjectileEvent;
use crate::{
    game::{audio::sfx::Sfx, enemy::Enemy, spawn::player::Player, Damageable},
    screen::GameState,
    AppSet,
};
use avian2d::collision::Collider;
use avian2d::prelude::{LinearVelocity, SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_invincibility_timer
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Running)),
            (handle_invincibility) //detect_enemy_player_collsion,
                .run_if(in_state(GameState::Running)),
        ),
    );
}

#[derive(Component)]
pub struct Invincibility {
    pub(crate) timer: Timer,
}
pub fn player_hit_by_projectile(
    trigger: Trigger<HitByProjectileEvent>,
    mut commands: Commands,
    mut death_state: ResMut<NextState<GameState>>,
    mut player_collision_query: Query<(&mut Damageable, &GlobalTransform), With<Player>>,
    spatial_query: SpatialQuery,
    mut enemy_velocities: Query<(&mut LinearVelocity, &GlobalTransform), With<Enemy>>,
) {
    info!("player hit by projectile");
    let player_entity = trigger.entity();
    let Ok((player_health, player_transform)) = player_collision_query.get_mut(player_entity)
    else {
        warn!("Player hit by projectiles called on non-player entity");
        return;
    };

    if player_health.health <= 0. {
        death_state.set(GameState::Death);
    }
    commands.trigger(Sfx::WizardGetsHit);

    // apply knockback to enemies when the player is hit
    let near_enemies = spatial_query.shape_intersections(
        &Collider::circle(100.),
        player_transform.translation().xy(),
        0.,
        SpatialQueryFilter::from_mask(GameLayer::Enemy),
    );

    for enemy in near_enemies.iter() {
        let Ok((mut velocity, transform)) = enemy_velocities.get_mut(*enemy) else {
            continue;
        };
        let direction = (transform.translation() - player_transform.translation())
            .xy()
            .normalize();
        velocity.0 = direction * 500.;
    }
}

// Detect enemy and player collide, take health away from player
// idk why clippy gets mad here, but i have to do this
// otherwise it broken sadge
// fn detect_enemy_player_collsion(
//     mut commands: Commands,
//     mut death_state: ResMut<NextState<GameState>>,
//     mut player_collision_query: Query<
//         (
//             &mut Damageable,
//             Entity,
//             &CollidingEntities,
//             Option<&Invincibility>,
//         ),
//         (With<Player>, Without<Enemy>),
//     >,
//     mut player_hit_knockback_caster: Query<(&PlayerHitKnockbackCaster, &ShapeHits)>,
//     enemy_query: Query<Entity, With<Enemy>>,
// ) {
//     for (mut player_health, player_entity, colliding_entities, invincibility) in
//         player_collision_query.iter_mut()
//     {
//         if invincibility.is_some() {
//             continue;
//         }
//
//         for &colliding_entity in colliding_entities.0.iter() {
//             if enemy_query.get(colliding_entity).is_ok() {
//                 player_health.health -= 1.0;
//                 println!("Player hit! Health: {:?}", player_health.0);
//                 if player_health.health <= 0. {
//                     death_state.set(GameState::Death);
//                 }
//                 commands.trigger(Sfx::WizardGetsHit);
//                 commands
//                     .entity(player_entity)
//                     .insert(Invincibility::new(5.0));
//
//                 // apply knockback to enemies when the player is hit
//                 for (_, hits) in player_hit_knockback_caster.iter_mut() {
//                     for hit in hits.iter() {
//                         // apply knockback to enemy based on distance and direction from player
//                         let Ok(direction) = Dir2::new(hit.normal2)
//                         else { continue; };
//                         let distance = hit.point1.length();
//
//                         commands.add(ApplyKnockback {
//                             entity: hit.entity,
//                             direction,
//                             strength: 100.0 / distance,
//                         });
//                     }
//                 }
//             }
//             break;
//         }
//     }
// }

fn tick_invincibility_timer(time: Res<Time>, mut query: Query<&mut Invincibility>) {
    for mut invincibility in query.iter_mut() {
        invincibility.timer.tick(time.delta());
    }
}

fn handle_invincibility(mut commands: Commands, mut query: Query<(Entity, &mut Invincibility)>) {
    for (entity, invincibility) in query.iter_mut() {
        if invincibility.timer.finished() {
            info!("Invincibility Removed");
            commands.entity(entity).remove::<Invincibility>();
        }
    }
}
