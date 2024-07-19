//! Gets the transform of the WorldBox and creates borders
//! based off its tranform

use bevy::{
    color::palettes::css::WHITE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::config::{BORDER_THICKNESS, MAP_HEIGHT, MAP_WIDTH};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_box_borders);
}

#[derive(Event, Debug)]
pub struct SpawnBorders;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Border;

fn spawn_box_borders(
    _trigger: Trigger<SpawnBorders>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let borders = [
        // Right border
        (
            Vec3::new(MAP_WIDTH / 2. + BORDER_THICKNESS / 2., 0., 0.),
            Vec2::new(BORDER_THICKNESS, MAP_HEIGHT).extend(0.0),
        ),
        // Left border
        (
            Vec3::new(-(MAP_WIDTH / 2. + BORDER_THICKNESS / 2.), 0., 0.),
            Vec2::new(BORDER_THICKNESS, MAP_HEIGHT).extend(0.0),
        ),
        // Top border
        (
            Vec3::new(0., MAP_HEIGHT / 2. + BORDER_THICKNESS / 2., 0.),
            Vec2::new(MAP_WIDTH + BORDER_THICKNESS * 2., BORDER_THICKNESS).extend(0.0),
        ),
        // Bottom border
        (
            Vec3::new(0., -(MAP_HEIGHT / 2. + BORDER_THICKNESS / 2.), 0.),
            Vec2::new(MAP_WIDTH + BORDER_THICKNESS * 2., BORDER_THICKNESS).extend(0.0),
        ),
    ];

    for (position, scale) in borders.iter() {
        commands.spawn((
            Name::new("Border"),
            Border,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
                transform: Transform::from_translation(*position).with_scale(*scale),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
    }
}
