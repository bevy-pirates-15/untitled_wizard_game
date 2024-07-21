//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{color::palettes::css::BLUE, dev_tools::states::log_transitions, prelude::*};
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::{GameState, Screen};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>)
        .add_systems(Update, log_transitions::<GameState>)
        .add_plugins((WorldInspectorPlugin::new(), PhysicsDebugPlugin::default()))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::from(BLUE)),
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_systems(Update, change_state_menu);
}

fn change_state_menu(
    mut contexts: EguiContexts, 
    mut set_state: ResMut<NextState<Screen>>, 
    mut set_sub_playing_state: ResMut<NextState<GameState>>
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Change Game State").show(ctx, |ui| {
        if ui.button("Splash").clicked() {
            set_state.set(Screen::Splash);
        }
        if ui.button("Title").clicked() {
            set_state.set(Screen::Title);
        }
        if ui.button("Credits").clicked() {
            set_state.set(Screen::Credits);
        }
        if ui.button("Playing").clicked() {
            set_state.set(Screen::Playing);
        }
        if ui.button("Running").clicked() {
            set_sub_playing_state.set(GameState::Running);
        }
        if ui.button("Paused").clicked() {
            set_sub_playing_state.set(GameState::Paused);
        }
        if ui.button("GemSelection").clicked() {
            set_sub_playing_state.set(GameState::GemSelection);
        }
    });
}