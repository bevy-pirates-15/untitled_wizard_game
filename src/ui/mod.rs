//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
pub mod palette;
mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

/// Insert into button to play the default interaction sounds
/// All buttons created from widgets.rs have this component
#[derive(Component, Debug)]
pub struct DefaultButtonSound;

#[derive(Component, Debug)]
pub struct GemPickUpButtonSound;

#[derive(Component, Debug)]
pub struct GemPlaceButtonSound;

#[derive(Component, Debug)]
pub struct GemDiscardButtonSound;
