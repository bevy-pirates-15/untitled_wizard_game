//! Pause menu is completely seperate from other menus
//! Runs if player presses "escape" on keyboard
//! TODO: Add ability for controller players to use this?

use super::{GameState, Screen};
use crate::ui::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Paused),
        enter_pause.run_if(in_state(Screen::Playing)),
    );

    app.register_type::<PauseAction>();
    app.add_systems(
        Update,
        handle_pause_action.run_if(in_state(GameState::Paused)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum PauseAction {
    Continue,
    Menu,
}

fn enter_pause(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(GameState::Paused))
        .with_children(|children| {
            children.button("Continue").insert(PauseAction::Continue);
            children
                .button("Quit to Main Menu")
                .insert(PauseAction::Menu);
        });
}

fn handle_pause_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_pause_state: ResMut<NextState<GameState>>,
    mut button_query: InteractionQuery<&PauseAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                PauseAction::Continue => {
                    next_pause_state.set(GameState::Running);
                }
                PauseAction::Menu => {
                    next_screen.set(Screen::Title);
                    next_pause_state.set(GameState::Running);
                }
            }
        }
    }
}
