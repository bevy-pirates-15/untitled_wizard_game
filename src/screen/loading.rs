//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::game::assets::particles::ParticleAssets;
use crate::game::assets::spell_gfx::SpellGFXAssets;
use crate::{
    game::assets::{ImageAssets, SfxAssets, SoundtrackAssets},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<LoadingState>();
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, check_all_loaded.run_if(in_state(Screen::Loading)));
}

fn enter_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Preload assets so the game runs smoothly.
    commands.insert_resource(ImageAssets::new(&asset_server));
    commands.insert_resource(SfxAssets::new(&asset_server));
    commands.insert_resource(SoundtrackAssets::new(&asset_server));
    commands.insert_resource(SpellGFXAssets::new(&asset_server));
    commands.insert_resource(ParticleAssets::new(&asset_server));
}

fn check_all_loaded(
    image_assets: Res<Assets<Image>>,
    audio_assets: Res<Assets<AudioSource>>,
    #[allow(dead_code)] _shader_assets: Res<Assets<Shader>>,
    images: Res<ImageAssets>,
    sfxs: Res<SfxAssets>,
    soundtracks: Res<SoundtrackAssets>,
    spellgfx: Res<SpellGFXAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_load_state: ResMut<NextState<LoadingState>>,
) {
    let all_loaded = images.all_loaded(&image_assets)
        && sfxs.all_loaded(&audio_assets)
        && soundtracks.all_loaded(&audio_assets)
        && spellgfx.all_loaded(&image_assets);
    if all_loaded {
        next_screen.set(Screen::Title);
        next_load_state.set(LoadingState::Loaded);
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
pub enum LoadingState {
    #[default]
    Loading,
    Loaded,
}
