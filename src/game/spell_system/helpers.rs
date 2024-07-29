use std::time::Duration;

use crate::game::assets::spell_gfx::{SpellGFXAsset, SpellGFXAssets};
use crate::game::physics::GameLayer;
use crate::game::projectiles::{ProjectileDamage, ProjectileLifetime, ProjectileTeam};
use crate::game::spell_system::casting::SpellCastContext;
use crate::screen::Screen;
use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody, Sensor};
use bevy::asset::Assets;
use bevy::log::warn;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    Entity, GlobalTransform, Mesh, SpatialBundle, StateScoped, Timer, TimerMode, Transform, World,
};
use bevy::sprite::{ColorMaterial, Mesh2dHandle, Sprite};

pub enum SpellModel {
    // None,
    StaticSprite(SpellGFXAsset),
    MeshMat(Mesh, ColorMaterial),
}

pub struct ProjectileStats {
    pub radius: f32,
    pub speed: f32,
    pub damage: f32,
    pub num_hits: i32,
    pub lifetime: Duration,
    pub knockback_force: f32,
}

pub fn spawn_spell_projectile(
    context: &mut SpellCastContext,
    world: &mut World,

    spell_model: SpellModel,
    stats: ProjectileStats,
) -> Option<Entity> {
    let Some(caster_transform) = world
        .entity(context.caster)
        .get::<GlobalTransform>()
        .map(|x| x.compute_transform())
    else {
        warn!("Tried to cast spell from an entity with no global transform");
        return None;
    };

    //create new spell entity:
    let spell = world
        .spawn((
            Collider::circle(stats.radius),
            RigidBody::Kinematic,
            Sensor,
            SpatialBundle {
                transform: Transform::from_translation(
                    caster_transform.translation + Vec3::new(0.0, 0.0, 0.1),
                )
                .with_rotation(Quat::from_rotation_z(
                    -context.direction.x.atan2(context.direction.y),
                ))
                .with_scale(Vec3::splat(1.0)),
                ..Default::default()
            },
            // LinearVelocity((spell_transform.rotation * Vec3::Y).truncate() * speed),
            LinearVelocity(context.direction * stats.speed),
            CollisionLayers::new(
                GameLayer::PlayerProjectile,
                [GameLayer::Environment, GameLayer::Enemy],
            ),
            ProjectileDamage {
                damage: stats.damage,
                hits_remaining: stats.num_hits,
                team: ProjectileTeam::Player,
                knockback_force: stats.knockback_force,
            },
            ProjectileLifetime {
                lifetime: Timer::new(stats.lifetime, TimerMode::Once),
            },
            StateScoped(Screen::Playing),
        ))
        .id();

    match spell_model {
        // SpellModel::None => {}
        SpellModel::StaticSprite(gfx) => {
            let gfx_assets = world.get_resource::<SpellGFXAssets>().unwrap();
            let sprite = gfx_assets[&gfx].clone_weak();
            world.entity_mut(spell).insert((Sprite::default(), sprite));
        }
        SpellModel::MeshMat(mesh, mat) => {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            let h_mesh: Mesh2dHandle = meshes.add(mesh).into();
            let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
            let h_mat = materials.add(mat);

            world.entity_mut(spell).insert((h_mesh, h_mat));
        }
    }

    //apply modifiers:
    context.values.modifiers.apply(spell, world);

    Some(spell)
}
