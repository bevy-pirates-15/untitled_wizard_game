pub mod sfx;
pub mod soundtrack;

use bevy::prelude::*;
use sfx::Sfx;

use crate::ui::{
    DefaultButtonSound, GemDiscardButtonSound, GemPickUpButtonSound, GemPlaceButtonSound,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            default_button_interaction_sfx,
            gem_pick_up_button_interaction_sfx,
            gem_place_button_interaction_sfx,
            gem_discard_button_interaction_sfx,
        ),
    );

    app.observe(soundtrack::play_soundtrack);
    app.observe(sfx::play_sfx);
}

/// If you create a button using widgets.rs, these sounds will play
fn default_button_interaction_sfx(
    mut interactions: Query<&'static Interaction, (Changed<Interaction>, With<DefaultButtonSound>)>,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(Sfx::ButtonHover),
            Interaction::Pressed => commands.trigger(Sfx::ButtonPress),
            _ => {}
        }
    }
}

fn gem_pick_up_button_interaction_sfx(
    mut interactions: Query<
        &'static Interaction,
        (Changed<Interaction>, With<GemPickUpButtonSound>),
    >,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(Sfx::ButtonHover),
            Interaction::Pressed => commands.trigger(Sfx::PickUpGem),
            _ => {}
        }
    }
}

fn gem_place_button_interaction_sfx(
    mut interactions: Query<
        &'static Interaction,
        (Changed<Interaction>, With<GemPlaceButtonSound>),
    >,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(Sfx::ButtonHover),
            Interaction::Pressed => commands.trigger(Sfx::PlaceGem),
            _ => {}
        }
    }
}

fn gem_discard_button_interaction_sfx(
    mut interactions: Query<
        &'static Interaction,
        (Changed<Interaction>, With<GemDiscardButtonSound>),
    >,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(Sfx::ButtonHover),
            Interaction::Pressed => commands.trigger(Sfx::DiscardGem),
            _ => {}
        }
    }
}
