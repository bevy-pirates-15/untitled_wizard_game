//! The screen state for the main game loop.

use bevy::color::palettes::tailwind::GREEN_400;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::{GameState, Screen};
use crate::game::levelling::{compute_next_level, PlayerLevel};
use crate::game::spawn::player::Player;
use crate::game::{audio::soundtrack::Soundtrack, spawn::map::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), (enter_playing, spawn_level_bar));
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(Update, update_level_bar.run_if(in_state(Screen::Playing)));
    app.add_systems(
        Update,
        (toggle_game_pause)
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands.trigger(Soundtrack::Gameplay);
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(Soundtrack::Disable);
}

#[derive(Component)]
struct LevelBar;

// Probably add a level bar here somewhere
fn spawn_level_bar(mut commands: Commands) {
    // TODO: check if bar is already here
    let ui_container = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Baseline,
            justify_content: JustifyContent::FlexStart,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    };

    let level_bar = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            ..default()
        },
        background_color: BackgroundColor(Color::from(GREEN_400)),
        ..default()
    };

    let ui_container_entity = commands
        .spawn(ui_container)
        .insert(StateScoped(Screen::Playing))
        .id();
    let level_bar_entity = commands.spawn(level_bar).insert(LevelBar).id();

    commands
        .entity(ui_container_entity)
        .push_children(&[level_bar_entity]);
}

fn update_level_bar(
    mut level_bar_query: Query<&mut Style, With<LevelBar>>,
    player_level_query: Query<&PlayerLevel, With<Player>>,
) {
    for mut style in &mut level_bar_query {
        if let Ok(player_level) = player_level_query.get_single() {
            let total_exp_in_level = compute_next_level(player_level.level);
            let percent_fill = ((total_exp_in_level as f32 - player_level.exp_to_level_up as f32)
                / total_exp_in_level as f32)
                * 100.;
            style.width = Val::Percent(percent_fill as f32);
        };
    }
}

fn toggle_game_pause(
    curr_pause_state: Res<State<GameState>>,
    mut next_pause_state: ResMut<NextState<GameState>>,
) {
    match curr_pause_state.get() {
        GameState::Paused => next_pause_state.set(GameState::Running),
        GameState::Running => next_pause_state.set(GameState::Paused),
        // Unable to pause game when in Gem Selection
        _ => {}
    }
}
