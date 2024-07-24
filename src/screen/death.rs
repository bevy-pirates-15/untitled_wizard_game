use bevy::prelude::*;

use super::{GameState, Screen};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Death),
        enter_death.run_if(in_state(GameState::Death))
    );

    app.register_type::<DeathAction>();
    app.add_systems(
        Update,
        handle_death_action.run_if(in_state(GameState::Death))
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum DeathAction {
    Menu,
}

fn enter_death(mut commands: Commands) {
    commands.ui_root()
        .insert(StateScoped(GameState::Death))
        .with_children(|children| {
            children
                .label("You Died.");
            children
                .button("Main Menu")
                .insert(DeathAction::Menu);
        });
}

fn handle_death_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut button_query: InteractionQuery<&DeathAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                DeathAction::Menu => {
                    next_screen.set(Screen::Title);
                    next_game_state.set(GameState::Running);
                }
            }
        }
    }
}