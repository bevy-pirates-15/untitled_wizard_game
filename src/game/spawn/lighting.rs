use bevy::{prelude::*, render::view::RenderLayers};
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
}
