use bevy::{
    color::palettes::css::BROWN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{game::aiming::PlayerAim, screen::Screen};

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
    commands.spawn((
        Name::new("Wand"),
        Wand,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
            transform: Transform::default().with_scale(Vec2::new(20., 70.).extend(2.0)),
            material: materials.add(Color::from(BROWN)),
            ..default()
        },
        PlayerAim::default(),
        StateScoped(Screen::Playing),
    ));
}
