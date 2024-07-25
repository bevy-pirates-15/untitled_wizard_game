use std::time::Duration;

use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody, Sensor};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::log::warn;
use bevy::math::Vec3;
use bevy::prelude::{Circle, Entity, GlobalTransform, Mesh, Timer, TimerMode, Transform, World};
use bevy::render::view::RenderLayers;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_magic_light_2d::gi::render_layer::ALL_LAYERS;
use bevy_magic_light_2d::prelude::{OmniLightSource2D, CAMERA_LAYER_OBJECTS};

use crate::game::physics::GameLayer;
use crate::game::projectiles::{ProjectileDamage, ProjectileLifetime, ProjectileTeam};
use crate::game::spell_system::casting::SpellCastContext;

pub fn spawn_spell_projectile(
    context: &mut SpellCastContext,
    world: &mut World,

    // stats:
    radius: f32,
    speed: f32,
    damage: f32,
    num_hits: i32,
    lifetime: Duration,
) -> Option<Entity> {
    let Some(caster_transform) = world
        .entity(context.caster)
        .get::<GlobalTransform>()
        .map(|x| x.compute_transform())
    else {
        warn!("Tried to cast spell from an entity with no global transform");
        return None;
    };

    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = meshes.add(Circle { radius });

    let col = Color::hsv(rand::random::<f32>() * 360., 1.0, 1.0);

    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
    let mat = materials.add(ColorMaterial::from(col));

    //create new spell entity:
    let spell = world
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle::from(mesh),
                material: mat,
                transform: Transform::from_translation(
                    caster_transform.translation + Vec3::new(0.0, 0.0, 0.1),
                ), // Transform::from_translation(.translation.with_z(4.0)).with
                ..Default::default()
            },
            Collider::circle(radius),
            RigidBody::Kinematic,
            Sensor,
            // LinearVelocity((spell_transform.rotation * Vec3::Y).truncate() * speed),
            LinearVelocity(context.direction * speed),
            CollisionLayers::new(
                GameLayer::PlayerProjectile,
                [GameLayer::Environment, GameLayer::Enemy],
            ),
            ProjectileDamage {
                damage,
                hits_remaining: num_hits,
                team: ProjectileTeam::Player,
            },
            ProjectileLifetime {
                lifetime: Timer::new(lifetime, TimerMode::Once),
            },
        ))
        .insert(RenderLayers::from_layers(ALL_LAYERS))
        .insert(OmniLightSource2D {
            intensity: 4.5,
            color: col,
            jitter_intensity: 2.5,
            jitter_translation: 3.0,
            falloff: Vec3::new(50.0, 20.0, 0.05),
        })
        .id();

    //apply modifiers:
    context.values.modifiers.apply(spell, world);

    Some(spell)
}
