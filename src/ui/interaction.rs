use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, apply_interaction_palette);
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: (Color, Color),
    pub hovered: (Color, Color),
    pub pressed: (Color, Color),
}

fn apply_interaction_palette(
    mut palette_query: InteractionQuery<(
        &InteractionPalette,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    for (interaction, (palette, mut background, mut border)) in &mut palette_query {
        *border = match interaction {
            Interaction::None => palette.none.1,
            Interaction::Hovered => palette.hovered.1,
            Interaction::Pressed => palette.pressed.1,
        }
        .into();
        *background = match interaction {
            Interaction::None => palette.none.0,
            Interaction::Hovered => palette.hovered.0,
            Interaction::Pressed => palette.pressed.0,
        }
        .into();
    }
}
