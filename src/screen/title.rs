//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::{GameState, Screen};
use crate::{game::audio::soundtrack::Soundtrack, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.add_systems(OnExit(Screen::Title), exit_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands) {
    commands.trigger(Soundtrack::MainMenu);
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").insert(TitleAction::Play);
            children.button("Credits").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
        });
}

fn exit_title(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(Soundtrack::Disable);
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_pause_state: ResMut<NextState<GameState>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => {
                    next_screen.set(Screen::Playing);
                    next_pause_state.set(GameState::Running);
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
