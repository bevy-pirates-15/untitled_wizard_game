use std::sync::Arc;

use bevy::{
    color::palettes::css::BROWN,
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_magic_light_2d::prelude::CAMERA_LAYER_OBJECTS;

use crate::game::spell_system::storage::RebuildWand;
use crate::game::spell_system::triggers::PlayerSpellTrigger;
use crate::game::spell_system::SpellModifierNode;
use crate::{
    game::{
        player_mods::aiming::{AttachToPlayer, PlayerAim},
        spell_system::casting::{CasterTargeter, SequentialCaster, SpellCastValues, SpellCaster},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_wand);
}

#[derive(Event, Debug)]
pub struct SpawnWand;

#[derive(Component, Debug, Default)]
pub struct Wand;

fn spawn_wand(
    _trigger: Trigger<SpawnWand>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut e = commands.spawn((
        Name::new("Wand"),
        Wand,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Rectangle::new(5., 40.)
                        .mesh()
                        .build()
                        .translated_by(Vec3::new(0.0, 10.0, 0.0)),
                ),
            ),
            // transform: Transform::default().with_scale(Vec2::new(20., 70.).extend(2.0)),
            material: materials.add(Color::from(BROWN)),
            ..default()
        },
        PlayerAim(Vec2::new(0.0, 1.0)),
        StateScoped(Screen::Playing),
        AttachToPlayer,
    ));
    e.insert(RenderLayers::from_layers(CAMERA_LAYER_OBJECTS));

    // wand_inventory.rebuild_effects();
    e.insert((
        SpellCaster::Sequential(SequentialCaster::new()),
        PlayerSpellTrigger {
            values: SpellCastValues {
                spread: 10.0,
                modifiers: Arc::new(SpellModifierNode::Root),
            },
            spells: Arc::new(vec![]),
        },
        CasterTargeter::RotationBased(Vec2::new(0.0, 1.0)),
    ));

    commands.trigger(RebuildWand);
}
