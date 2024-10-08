use bevy::{prelude::*, window::WindowMode};

use crate::{
    game::assets::{ImageAsset, ImageAssets},
    ui::prelude::InteractionQuery,
};

use super::loading::LoadingState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LoadingState::Loaded), spawn_fullscreen_icon);
    app.add_systems(
        Update,
        (handle_fullscreen_button, handle_fullscreen_boolean).chain(),
    );
}

#[derive(Component)]
struct InFullScreen(bool);

fn spawn_fullscreen_icon(mut commands: Commands, images: Res<ImageAssets>) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .id();

    let fullscreen_button_entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(6.5),
                    height: Val::Percent(10.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Percent(0.5)),
                    ..default()
                },
                ..default()
            },
            InFullScreen(false),
        ))
        .id();

    let fullscreen_image_entity = commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: images[&ImageAsset::FullScreen].clone_weak(),
                color: Color::srgba(1.0, 1.0, 1.0, 0.2),
                ..default()
            },
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .entity(root_entity)
        .push_children(&[fullscreen_button_entity]);
    commands
        .entity(fullscreen_button_entity)
        .push_children(&[fullscreen_image_entity]);
}

fn handle_fullscreen_button(
    fullscreen_button_query: InteractionQuery<&InFullScreen>,
    mut windows: Query<&mut Window>,
) {
    for (interaction, toggle_fullscreen) in &mut fullscreen_button_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            let mut window = windows.single_mut();
            match toggle_fullscreen.0 {
                false => window.mode = WindowMode::BorderlessFullscreen,
                true => window.mode = WindowMode::Windowed,
            }
        }
    }
}

fn handle_fullscreen_boolean(
    mut is_in_full_screen_query: Query<&mut InFullScreen>,
    windows: Query<&Window>,
) {
    if let Ok(mut toggle_fullscreen) = is_in_full_screen_query.get_single_mut() {
        let window = windows.single();
        // clippy forced me to put this here
        // Basically if borderlessfullscreen is on set to true
        // else it is false
        toggle_fullscreen.0 = window.mode == WindowMode::BorderlessFullscreen;
    }
}
