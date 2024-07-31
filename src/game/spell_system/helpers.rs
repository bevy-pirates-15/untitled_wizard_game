use std::time::Duration;

use crate::game::assets::particles::{ParticleAsset, ParticleAssets};
use crate::game::assets::spell_gfx::{SpellGFXAsset, SpellGFXAssets};
use crate::game::lighting::GameLight;
use crate::game::projectiles::{ProjectileDamage, ProjectileLifetime, ProjectileTeam};
use crate::game::spell_system::casting::SpellCastContext;
use crate::screen::Screen;
use avian2d::prelude::{Collider, LinearVelocity, RigidBody, Sensor};
use bevy::asset::Assets;
use bevy::log::{info, warn};
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::prelude::{
    Entity, GlobalTransform, Mesh, SpatialBundle, StateScoped, Timer, TimerMode, Transform, World,
};
use bevy::sprite::{ColorMaterial, Mesh2dHandle, Sprite};
use bevy_particle_systems::{BurstIndex, ParticleCount, Playing, RunningState};

pub enum SpellModel {
    None,
    StaticSprite(SpellGFXAsset),
    #[allow(dead_code)]
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
    team: ProjectileTeam,
    spell_model: SpellModel,
    spell_particles: Option<ParticleAsset>,
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

    /*
    let (z, _, _) = g_transform
            .compute_transform()
            .rotation
            .to_euler(EulerRot::ZXY);

     direction: Vec2::new(-z.sin(), z.cos()),
     */

    //calculate new rotation using context direction and context spread
    let direction = -context.direction.x.atan2(context.direction.y);
    //spread is degrees of spread, so we need to convert to radians
    let spread = context.values.spread.to_radians();
    let rotation = Quat::from_rotation_z(direction + spread * ((rand::random::<f32>() - 0.5) * 2.));

    let (z, _, _) = rotation.to_euler(EulerRot::ZXY);

    //convert rotation back into a direction vector for the velocity
    let vel_vec = Vec2::new(-z.sin(), z.cos());

    //create new spell entity:
    let spell = world
        .spawn((
            Collider::circle(stats.radius),
            RigidBody::Kinematic,
            Sensor,
            SpatialBundle {
                transform: Transform::from_translation(
                    caster_transform.translation + Vec3::new(0.0, 0.0, 30.),
                )
                .with_rotation(rotation)
                .with_scale(Vec3::splat(1.0)),
                ..Default::default()
            },
            // LinearVelocity((spell_transform.rotation * Vec3::Y).truncate() * speed),
            LinearVelocity(vel_vec * stats.speed),
            team.get_collision_layer(),
            ProjectileDamage {
                damage: stats.damage,
                hits_remaining: stats.num_hits,
                team: team.clone(),
                knockback_force: stats.knockback_force,
            },
            ProjectileLifetime {
                lifetime: Timer::new(stats.lifetime, TimerMode::Once),
            },
            StateScoped(Screen::Playing),
        ))
        .id();

    if team == ProjectileTeam::Player {
        world.entity_mut(spell).insert(GameLight {
            radius: 20.0,
            priority: 10,
        });
    }

    match spell_model {
        SpellModel::None => {}
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

    //add particles if not None
    if let Some(particle) = spell_particles {
        info!("Adding particles to spell");
        let particles = world.get_resource_mut::<ParticleAssets>().unwrap()[&particle].clone();
        world.entity_mut(spell).insert((
            particles,
            ParticleCount::default(),
            RunningState::default(),
            BurstIndex::default(),
            Playing,
        ));
    }

    //apply modifiers:
    context.values.modifiers.apply(spell, world);

    Some(spell)
}
