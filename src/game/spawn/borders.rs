//! Gets the transform of the WorldBox and creates borders
//! based off its tranform

use avian2d::prelude::CollisionLayers;
use avian2d::{
    collision::Collider,
    prelude::{LockedAxes, RigidBody},
};
use bevy::prelude::*;

use crate::config::{BORDER_THICKNESS, MAP_HEIGHT, MAP_WIDTH};
use crate::game::physics::GameLayer;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_box_borders);
}

#[derive(Event, Debug)]
pub struct SpawnBorders;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Border;

fn spawn_box_borders(_trigger: Trigger<SpawnBorders>, mut commands: Commands) {
    let borders = [
        // Right border
        (
            Vec3::new(MAP_WIDTH / 2. + BORDER_THICKNESS / 2., 0., 0.),
            Vec2::new(BORDER_THICKNESS, MAP_HEIGHT),
        ),
        // Left border
        (
            Vec3::new(-(MAP_WIDTH / 2. + BORDER_THICKNESS / 2.), 0., 0.),
            Vec2::new(BORDER_THICKNESS, MAP_HEIGHT),
        ),
        // Top border
        (
            Vec3::new(0., MAP_HEIGHT / 2. + BORDER_THICKNESS / 2., 0.),
            Vec2::new(MAP_WIDTH + BORDER_THICKNESS * 2., BORDER_THICKNESS),
        ),
        // Bottom border
        (
            Vec3::new(0., -(MAP_HEIGHT / 2. + BORDER_THICKNESS / 2.), 0.),
            Vec2::new(MAP_WIDTH + BORDER_THICKNESS * 2., BORDER_THICKNESS),
        ),
    ];

    for (position, scale) in borders.iter() {
        commands.spawn((
            Name::new("Border"),
            Border,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.7, 0.7, 0.8),
                    custom_size: Some(*scale),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, position.z),
                ..default()
            },
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Static,
            Collider::rectangle(scale.x, scale.y),
            CollisionLayers::new(GameLayer::Border, [GameLayer::Player]),
            StateScoped(Screen::Playing),
        ));
    }
}
