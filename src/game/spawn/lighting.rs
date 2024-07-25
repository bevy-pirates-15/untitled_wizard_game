use bevy::{
    color::palettes::css::YELLOW, prelude::*, render::view::RenderLayers,
    sprite::MaterialMesh2dBundle,
};
use bevy_magic_light_2d::{gi::render_layer::ALL_LAYERS, prelude::*};

use crate::game::player_mods::aiming::AttachToPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_light)
        .register_type::<OmniLightSource2D>();
}

#[derive(Event, Debug)]
pub struct SpawnLight;

#[derive(Component, Debug, Default)]
pub struct Light;

fn spawn_light(
    _trigger: Trigger<SpawnLight>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    warn!("Lighting!");

    // Add skylight light.
    commands.spawn((
        SkylightLight2D {
            color: Color::srgb_u8(93, 158, 179),
            intensity: 0.025,
        },
        Name::new("global_skylight"),
    ));

    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1000.0),
                scale: Vec3::splat(8.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("cursor_light"))
        .insert(OmniLightSource2D {
            intensity: 10.0,
            color: Color::srgb_u8(254, 100, 34),
            falloff: Vec3::new(5.0, 5.0, 0.05),
            ..default()
        })
        .insert(RenderLayers::from_layers(ALL_LAYERS))
        .insert(AttachToPlayer);
}
