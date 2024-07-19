//! Spawn the main level by triggering other observers.

use bevy::{
    color::palettes::css::PURPLE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::{enemy::SpawnEnemies, player::SpawnPlayer, wand::SpawnWand};
use crate::config::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Map"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(MAP_WIDTH, MAP_HEIGHT))),
            transform: Transform::default().with_scale(Vec2::splat(420.).extend(0.0)),
            material: materials.add(Color::from(PURPLE)),
            ..default()
        }
    ));
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnWand);
    commands.trigger(SpawnEnemies);
}
