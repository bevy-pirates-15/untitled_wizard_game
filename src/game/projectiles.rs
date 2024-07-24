use avian2d::collision::CollidingEntities;
use bevy::prelude::{
    App, Commands, Component, Entity, Event, IntoSystemConfigs, Query, Res, Time, Timer, Update,
    With, Without,
};

use crate::game::enemy::Enemy;
use crate::game::spawn::player::Player;
use crate::game::Health;
use crate::AppSet;

use super::audio::sfx::Sfx;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_projectile_lifetime.in_set(AppSet::TickTimers),
            (
                detect_projectile_collisions,
                despawn_projectiles_no_hits,
                despawn_projectiles_lifetime,
            )
                .in_set(AppSet::Update),
        ),
    );
}

#[derive(Event, Debug, Clone)]
pub struct ProjectileCollisionEvent {
    #[allow(dead_code)]
    pub target: Entity,
}

#[derive(Clone, Debug)]
pub enum ProjectileTeam {
    Player,
    #[allow(dead_code)]
    Enemy,
    #[allow(dead_code)]
    Neither,
}

#[derive(Component)]
pub struct ProjectileDamage {
    pub team: ProjectileTeam,
    pub damage: f32,
    pub hits_remaining: i32, //counter for how many enemies it can hit
}

#[derive(Component)]
pub struct ProjectileLifetime {
    pub lifetime: Timer,
}

fn tick_projectile_lifetime(time: Res<Time>, mut projectile_query: Query<&mut ProjectileLifetime>) {
    for mut projectile_data in projectile_query.iter_mut() {
        projectile_data.lifetime.tick(time.delta());
    }
}

fn despawn_projectiles_lifetime(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &ProjectileLifetime)>,
) {
    //despawn if pierce = 0 or lifetime is up
    for (entity, lifetime) in projectile_query.iter_mut() {
        if lifetime.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_projectiles_no_hits(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &ProjectileDamage)>,
) {
    //despawn if pierce = 0 or lifetime is up
    for (entity, dmg) in projectile_query.iter_mut() {
        if dmg.hits_remaining <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

// // code for only projectile hitting enemy
// fn detect_projectile_collisions (
//     mut commands: Commands,
//     mut projectile_query: Query<(Entity, &mut ProjectileDamage, &CollidingEntities)>,
//     mut enemy_query: Query<&mut Health, (With<Enemy>, Without<Player>)>,
// ) {
//     for (entity, mut projectile_damage, colliding_entities) in projectile_query.iter_mut() {
//         for &colliding_entity in colliding_entities.0.iter() {
//             if let Ok(mut health) = enemy_query.get_mut(colliding_entity) {
//                 //do damage
//                 health.0 -= projectile_damage.damage;

//                 //reduce pierce counter
//                 projectile_damage.hits_remaining -= 1;
//                 commands.trigger(Sfx::EnemyCollision);
//                 commands.trigger_targets(
//                     ProjectileCollisionEvent {
//                         target: colliding_entity,
//                     },
//                     entity,
//                 );
//             }
//         }
//     }
// }

// This breaks player/enemy collision donno why
fn detect_projectile_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut ProjectileDamage, &CollidingEntities)>,
    mut enemy_query: Query<&mut Health, (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&mut Health, (With<Player>, Without<Enemy>)>,
) {
    for (e, mut projectile_dmg, colliding_entities) in projectile_query.iter_mut() {
        for &colliding_entity in colliding_entities.0.iter() {
            let Ok(mut health) = (match &projectile_dmg.team {
                ProjectileTeam::Player => enemy_query.get_mut(colliding_entity),
                ProjectileTeam::Enemy => player_query.get_mut(colliding_entity),
                ProjectileTeam::Neither => enemy_query
                    .get_mut(colliding_entity)
                    .or_else(|_| player_query.get_mut(colliding_entity)),
            }) else {
                continue;
            };

            //do damage
            health.0 -= projectile_dmg.damage;

            //reduce pierce counter
            projectile_dmg.hits_remaining -= 1;

            //todo: OnHitTrigger
            commands.trigger(Sfx::EnemyCollision);
            commands.trigger_targets(
                ProjectileCollisionEvent {
                    target: colliding_entity,
                },
                e,
            );
        }
    }
}
