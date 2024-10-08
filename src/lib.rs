mod config;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui;

use avian2d::PhysicsPlugins;
use bevy::{
    asset::{load_internal_binary_asset, AssetMetaCheck},
    audio::{AudioPlugin, Volume},
    prelude::*,
    render::camera::ScalingMode,
    window::WindowResolution,
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_particle_systems::ParticleSystemPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "wizard_game".to_string(),
                        canvas: Some("#bevy".to_string()),
                        resolution: WindowResolution::new(1920., 1080.)
                            .with_scale_factor_override(1.0),
                        resizable: false,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        load_internal_binary_asset!(
            app,
            TextStyle::default().font,
            "../assets/fonts/IGS_VGA_8x16.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        // Add physics
        app.add_plugins(PhysicsPlugins::default().with_length_unit(20.));

        // Add tilemaps
        app.add_plugins(TilemapPlugin);

        // Add Particles
        app.add_plugins(ParticleSystemPlugin);

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, ui::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 640.,
        height: 360.,
    };

    commands.spawn((Name::new("Camera"), camera));
}
