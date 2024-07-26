use bevy::asset::{AssetServer, Assets, Handle};
use bevy::prelude::{Deref, DerefMut, Image, Reflect, Resource};
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::utils::HashMap;

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SpellGFXAsset {
    Zap,
    // Bang,
    ArcaneArrow,
    SplitterBolts,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SpellGFXAssets(HashMap<SpellGFXAsset, Handle<Image>>);

impl SpellGFXAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();

        assets.insert(
            SpellGFXAsset::Zap,
            asset_server.load_with_settings(
                "images/spell_gfx/zap.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        );
        // assets.insert(
        //     SpellGFXAsset::Bang,
        //     asset_server.load_with_settings(
        //         "images/spell_gfx/bang.png",
        //         |settings: &mut ImageLoaderSettings| {
        //             settings.sampler = ImageSampler::nearest();
        //         },
        //     ),
        // );
        assets.insert(
            SpellGFXAsset::ArcaneArrow,
            asset_server.load_with_settings(
                "images/spell_gfx/arcane_arrow.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        );
        assets.insert(
            SpellGFXAsset::SplitterBolts,
            asset_server.load_with_settings(
                "images/spell_gfx/split.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        );

        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<Image>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}
