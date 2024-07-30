use std::cmp::PartialEq;

use avian2d::prelude::{Collision, CollisionLayers, LinearVelocity};
use bevy::math::Vec3Swizzles;
use bevy::prelude::{
    in_state, App, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader,
    GlobalTransform, IntoSystemConfigs, Query, Reflect, Res, Time, Timer, TimerMode, Update,
};

use super::audio::sfx::Sfx;
use crate::game::physics::GameLayer;
use crate::game::player_mods::damage::Invincibility;
use crate::game::Damageable;
use crate::screen::GameState;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_projectile_lifetime
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Running)),
            (
                detect_projectile_collisions,
                despawn_projectiles_no_hits,
                despawn_projectiles_lifetime,
            )
                .in_set(AppSet::Update),
        ),
    );
    app.register_type::<ProjectileTeam>();
}

#[derive(Event, Debug, Clone)]
pub struct ProjectileCollisionEvent {
    #[allow(dead_code)]
    pub target: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct HitByProjectileEvent {
    #[allow(dead_code)]
    pub projectile: Entity,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum ProjectileTeam {
    Player,
    #[allow(dead_code)]
    Enemy,
}
impl ProjectileTeam {
    pub fn get_collision_layer(&self) -> CollisionLayers {
        match self {
            ProjectileTeam::Player => CollisionLayers::new(
                GameLayer::PlayerProjectile,
                [GameLayer::Environment, GameLayer::Enemy],
            ),
            ProjectileTeam::Enemy => CollisionLayers::new(
                GameLayer::EnemyProjectile,
                [GameLayer::Environment, GameLayer::Player],
            ),
        }
    }
}

#[derive(Component)]
pub struct ProjectileDamage {
    pub team: ProjectileTeam,
    pub damage: f32,
    pub hits_remaining: i32, //counter for how many enemies it can hit
    pub knockback_force: f32,
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
            commands.entity(entity).despawn_recursive();
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
            commands.entity(entity).despawn_recursive();
        }
    }
}

// This breaks player/enemy collision donno why
fn detect_projectile_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut commands: Commands,
    mut projectile_query: Query<(&GlobalTransform, &mut ProjectileDamage)>,
    mut health_havers: Query<(
        &GlobalTransform,
        Option<&mut LinearVelocity>,
        &mut Damageable,
        Option<&Invincibility>,
    )>,
) {
    //datastructure to keep track of hit entities, as they cant be hit more than once per frame
    let mut hit_entities = Vec::new();

    for Collision(contacts) in collision_event_reader.read() {
        let (Some(col1), Some(col2)) = (contacts.body_entity1, contacts.body_entity2) else {
            continue;
        };

        //check if one is the projectile and the other the health_haver
        let (projectile_entity, health_entity) =
            if projectile_query.get(col1).is_ok() && health_havers.get(col2).is_ok() {
                (col1, col2)
            } else if projectile_query.get(col2).is_ok() && health_havers.get(col1).is_ok() {
                (col2, col1)
            } else {
                continue;
            };

        let Ok((h_transform, lv, mut health, invincibility)) = health_havers.get_mut(health_entity)
        else {
            return;
        };

        let Ok((p_transform, mut projectile_dmg)) = projectile_query.get_mut(projectile_entity)
        else {
            return;
        };

        //can this projectile damage this entity?
        if health.team == projectile_dmg.team {
            continue;
        }

        //is this entity invulnerable
        if invincibility.is_some() {
            continue;
        }

        //has this entity already been hit this frame?
        if hit_entities.contains(&health_entity) {
            continue;
        }

        hit_entities.push(health_entity);

        //do damage + health.invincibility_timer)
        health.health -= projectile_dmg.damage;
        if let Some(timer) = health.invincibility_timer {
            commands.entity(health_entity).insert(Invincibility {
                timer: Timer::new(timer, TimerMode::Once),
            });
        }

        // apply knockback to enemies when the player is hit
        //get direction between projectile and health entity
        let direction = (h_transform.translation() - p_transform.translation())
            .xy()
            .normalize();
        if let Some(mut velocity) = lv {
            velocity.0 = direction * projectile_dmg.knockback_force;
        }

        //reduce projectile pierce counter
        projectile_dmg.hits_remaining -= 1;

        commands.trigger(Sfx::EnemyCollision);
        commands.trigger_targets(
            ProjectileCollisionEvent {
                target: health_entity,
            },
            projectile_entity,
        );
        commands.trigger_targets(
            HitByProjectileEvent {
                projectile: projectile_entity,
            },
            health_entity,
        );
    }
}
