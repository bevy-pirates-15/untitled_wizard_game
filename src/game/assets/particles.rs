use bevy::asset::AssetServer;
use bevy::color::palettes::basic::WHITE;
use bevy::color::Color;
use bevy::prelude::{Deref, DerefMut, Reflect, Resource};
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::utils::HashMap;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleBurst,
    ParticleSystem,
};

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum ParticleAsset {
    Zap,
    Bang,
    ArcaneArrow,
    SplitterBolts,
    Enemy,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct ParticleAssets(HashMap<ParticleAsset, ParticleSystem>);

impl ParticleAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();

        assets.insert(ParticleAsset::Zap, zap_particles(asset_server));
        assets.insert(ParticleAsset::Bang, bang_particles(asset_server));
        assets.insert(ParticleAsset::ArcaneArrow, arrow_particles(asset_server));
        assets.insert(
            ParticleAsset::SplitterBolts,
            splitter_particles(asset_server),
        );
        assets.insert(ParticleAsset::Enemy, enemy_particles(asset_server));

        Self(assets)
    }
}

fn zap_particles(asset_server: &AssetServer) -> ParticleSystem {
    ParticleSystem {
        max_particles: 500,
        texture: asset_server
            .load_with_settings("images/px.png", |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .into(),
        spawn_rate_per_second: 50.0.into(),
        initial_speed: JitteredValue::jittered(10.0, -10.0..10.0),
        // velocity_modifiers: vec![Drag(0.01.into())],
        lifetime: JitteredValue::new(0.25),
        color: ColorOverTime::Gradient(Curve::new(vec![
            CurvePoint::new(WHITE.into(), 0.0),
            CurvePoint::new(Color::srgba(0.0, 1.0, 1.0, 1.0), 0.5),
            CurvePoint::new(Color::srgba(0.0, 1.0, 1.0, 0.0), 1.0),
        ])),
        looping: true,
        system_duration_seconds: 2.0,
        max_distance: Some(50.0),
        // scale: 1.0.into(),
        // bursts: vec![
        //     ParticleBurst::new(0.0, 1000),
        //     ParticleBurst::new(2.0, 1000),
        //     ParticleBurst::new(4.0, 1000),
        //     ParticleBurst::new(6.0, 1000),
        //     ParticleBurst::new(8.0, 1000),
        // ],
        ..ParticleSystem::default()
    }
}

fn arrow_particles(asset_server: &AssetServer) -> ParticleSystem {
    ParticleSystem {
        max_particles: 500,
        texture: asset_server
            .load_with_settings("images/px.png", |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .into(),
        spawn_rate_per_second: 50.0.into(),
        initial_speed: JitteredValue::jittered(10.0, -10.0..10.0),
        // velocity_modifiers: vec![Drag(0.01.into())],
        lifetime: JitteredValue::new(0.25),
        color: ColorOverTime::Gradient(Curve::new(vec![
            CurvePoint::new(WHITE.into(), 0.0),
            CurvePoint::new(Color::srgba(0.0, 1.0, 0.0, 1.0), 0.5),
            CurvePoint::new(Color::srgba(1.0, 1.0, 0.0, 0.0), 1.0),
        ])),
        looping: true,
        system_duration_seconds: 2.0,
        max_distance: Some(50.0),
        ..ParticleSystem::default()
    }
}

fn splitter_particles(asset_server: &AssetServer) -> ParticleSystem {
    ParticleSystem {
        max_particles: 500,
        texture: asset_server
            .load_with_settings("images/px.png", |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .into(),
        spawn_rate_per_second: 50.0.into(),
        initial_speed: JitteredValue::jittered(10.0, -10.0..10.0),
        // velocity_modifiers: vec![Drag(0.01.into())],
        lifetime: JitteredValue::new(0.25),
        color: ColorOverTime::Gradient(Curve::new(vec![
            CurvePoint::new(WHITE.into(), 0.0),
            CurvePoint::new(Color::srgba(1.0, 0.5, 0.0, 1.0), 0.5),
            CurvePoint::new(Color::srgba(1.0, 1.0, 0.0, 0.0), 1.0),
        ])),
        looping: true,
        system_duration_seconds: 2.0,
        max_distance: Some(50.0),
        ..ParticleSystem::default()
    }
}

fn bang_particles(asset_server: &AssetServer) -> ParticleSystem {
    ParticleSystem {
        max_particles: 1000,
        texture: asset_server
            .load_with_settings("images/px.png", |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .into(),
        emitter_shape: EmitterShape::CircleSegment(CircleSegment {
            radius: JitteredValue::jittered(0.0, 0.0..30.0),
            ..CircleSegment::default()
        }),
        spawn_rate_per_second: 50.0.into(),
        initial_speed: JitteredValue::jittered(100.0, -10.0..10.0),
        // velocity_modifiers: vec![Drag(0.01.into())],
        lifetime: JitteredValue::new(0.25),
        color: ColorOverTime::Gradient(Curve::new(vec![
            CurvePoint::new(WHITE.into(), 0.0),
            CurvePoint::new(Color::srgba(1.0, 0.5, 0.0, 1.0), 0.5),
            CurvePoint::new(Color::srgba(1.0, 1.0, 0.0, 0.0), 1.0),
        ])),
        looping: true,
        system_duration_seconds: 2.0,
        max_distance: Some(100.0),
        bursts: vec![ParticleBurst::new(0.0, 1000)],
        ..ParticleSystem::default()
    }
}

fn enemy_particles(asset_server: &AssetServer) -> ParticleSystem {
    ParticleSystem {
        max_particles: 500,
        texture: asset_server
            .load_with_settings("images/px.png", |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .into(),
        spawn_rate_per_second: 50.0.into(),
        initial_speed: JitteredValue::jittered(10.0, -10.0..10.0),
        // velocity_modifiers: vec![Drag(0.01.into())],
        lifetime: JitteredValue::new(0.25),
        color: ColorOverTime::Gradient(Curve::new(vec![
            CurvePoint::new(Color::srgba_u8(255, 93, 204, 255), 0.0),
            CurvePoint::new(Color::srgba(0.0, 0.0, 0.0, 1.0), 0.5),
            CurvePoint::new(Color::srgba(0.0, 0.0, 0.0, 0.0), 1.0),
        ])),
        looping: true,
        system_duration_seconds: 2.0,
        max_distance: Some(50.0),
        // scale: 1.0.into(),
        // bursts: vec![
        //     ParticleBurst::new(0.0, 1000),
        //     ParticleBurst::new(2.0, 1000),
        //     ParticleBurst::new(4.0, 1000),
        //     ParticleBurst::new(6.0, 1000),
        //     ParticleBurst::new(8.0, 1000),
        // ],
        ..ParticleSystem::default()
    }
}
