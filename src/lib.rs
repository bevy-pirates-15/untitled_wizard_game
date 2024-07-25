mod config;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui;

use avian2d::PhysicsPlugins;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        texture::{ImageFilterMode, ImageSamplerDescriptor},
        view::RenderLayers,
    },
    window::WindowResolution,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_magic_light_2d::prelude::*;

pub const TILE_SIZE: f32 = 16.0;
pub const SPRITE_SCALE: f32 = 4.0;
pub const Z_BASE_FLOOR: f32 = 100.0; // Base z-coordinate for 2D layers.
pub const Z_BASE_OBJECTS: f32 = 200.0; // Ground object sprites.
pub const SCREEN_SIZE: (f32, f32) = (1280.0, 720.0);
pub const CAMERA_SCALE: f32 = 1.0;
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Add Bevy plugins.
        app.add_plugins((
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
                        resolution: WindowResolution::from(SCREEN_SIZE)
                            .with_scale_factor_override(1.0),
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
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        mag_filter: ImageFilterMode::Nearest,
                        min_filter: ImageFilterMode::Nearest,
                        ..default()
                    },
                }),
            BevyMagicLight2DPlugin,
            ResourceInspectorPlugin::<BevyMagicLight2DSettings>::new(),
        ));
        // Add physics
        app.add_plugins(PhysicsPlugins::default().with_length_unit(20.));

        // Add other plugins.
        app.add_plugins((game::plugin, screen::plugin, ui::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);

        app.insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 16,
                smooth_kernel_size: (2, 1),
                direct_light_contrib: 0.2,
                indirect_light_contrib: 0.8,
                ..default()
            },
            ..default()
        })
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .add_systems(Startup, spawn_camera.after(setup_post_processing_camera));
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

fn spawn_camera(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 640.,
        height: 360.,
    };

    let projection = OrthographicProjection {
        scale: CAMERA_SCALE,
        near: -2000.0,
        far: 2000.0,
        scaling_mode: ScalingMode::Fixed {
            width: 640.,
            height: 360.,
        },
        ..default()
    };

    commands.spawn((Name::new("Camera"), camera));
    // Setup separate camera for floor, walls and objects.
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: false,
                    target: RenderTarget::Image(camera_targets.floor_target.clone()),
                    ..default()
                },
                projection: projection.clone(),
                ..default()
            },
            Name::new("floors_target_camera"),
        ))
        .insert(SpriteCamera)
        .insert(FloorCamera)
        .insert(RenderLayers::from_layers(CAMERA_LAYER_FLOOR));
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: false,
                    target: RenderTarget::Image(camera_targets.walls_target.clone()),
                    ..default()
                },
                projection: projection.clone(),
                ..default()
            },
            Name::new("walls_target_camera"),
        ))
        .insert(SpriteCamera)
        .insert(WallsCamera)
        .insert(RenderLayers::from_layers(CAMERA_LAYER_WALLS));
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: false,
                    target: RenderTarget::Image(camera_targets.objects_target.clone()),
                    ..default()
                },
                projection: projection.clone(),
                ..default()
            },
            Name::new("objects_targets_camera"),
        ))
        .insert(SpriteCamera)
        .insert(ObjectsCamera)
        .insert(RenderLayers::from_layers(CAMERA_LAYER_OBJECTS));
}
