//! Spawn the main level by triggering other observers.

use bevy::{
    color::palettes::css::PURPLE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::screen::Screen;

use super::{borders::SpawnBorders, player::SpawnPlayer, wand::SpawnWand};
use crate::config::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct WorldBox;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn level box here, change to very pretty art later
    commands.spawn((
        Name::new("World Box"),
        WorldBox,
        MaterialMesh2dBundle {
            // IMPORTANT: The mesh itself needs to be {1.0, 1.0} (default)
            // Only use "Transform" to manipulate tranform
            // Otherwise, the math is all off :(
            mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
            transform: Transform::default()
                .with_scale(Vec2::new(MAP_WIDTH, MAP_HEIGHT).extend(0.0)),
            material: materials.add(Color::from(PURPLE)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnBorders);
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnWand);
    // commands.trigger(StartWave);
}
