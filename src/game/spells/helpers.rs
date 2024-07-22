use avian2d::prelude::{Collider, LinearVelocity, RigidBody, Sensor};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::{
    Circle, Entity, GlobalTransform, Mesh, Transform, World,
};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};

use crate::game::projectiles::{ProjectileDamage, ProjectileTeam};
use crate::game::spells::casting::SpellCastContext;

pub fn spawn_player_projectile(
    context: &mut SpellCastContext,
    world: &mut World,

    // stats:
    radius: f32,
    speed: f32,
    damage: f32,
) -> Option<Entity> {
    let Some(caster_gt) = world.entity(context.caster).get::<GlobalTransform>() else {
        println!("Tried to cast spell from an entity with no global transform");
        return None;
    };
    let spell_transform = caster_gt.compute_transform();

    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = meshes.add(Circle { radius });
    drop(meshes);

    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
    let mat = materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0)));
    drop(materials);

    //create new spell entity:
    let spell = world
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle::from(mesh),
                material: mat,
                transform: Transform::from_translation(spell_transform.translation.with_z(4.0)),
                ..Default::default()
            },
            Collider::circle(radius),
            RigidBody::Dynamic,
            Sensor,
            LinearVelocity((spell_transform.rotation * Vec3::Y).truncate() * speed),
            ProjectileDamage {
                team: ProjectileTeam::Player,
                damage,
            },
        ))
        .id();

    //apply modifiers:
    context.modifiers.apply(spell, world);

    return Some(spell);
}
