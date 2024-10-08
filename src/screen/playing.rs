//! The screen state for the main game loop.

use bevy::color::palettes::css::{DARK_RED, RED};
use bevy::color::palettes::tailwind::GREEN_400;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::{GameState, Screen};
use crate::game::{
    audio::soundtrack::Soundtrack,
    enemy::{StartWave, Wave, WaveState, WaveText},
    levelling::{compute_next_level, LevelText, PlayerLevel},
    spawn::map::SpawnLevel,
    spawn::player::Player,
    Damageable,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), (enter_playing, spawn_playing_gui));
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        (
            update_level_bar,
            update_health_bar,
            update_wave_bar.run_if(resource_equals(WaveState::Active)),
        )
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        Update,
        (toggle_game_pause)
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands.trigger(StartWave);
    commands.trigger(Soundtrack::Gameplay);
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(Soundtrack::Disable);
}

#[derive(Component)]
struct LevelBar;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct WaveBar;

fn spawn_playing_gui(mut commands: Commands) {
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
            height: Val::Percent(5.),
            ..default()
        },
        background_color: BackgroundColor(Color::from(GREEN_400)),
        ..default()
    };

    let health_bar = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(2.),
            ..default()
        },
        background_color: BackgroundColor(Color::from(RED)),
        ..default()
    };

    let level_text = TextBundle {
        style: Style {
            width: Val::Percent(50.),
            height: Val::Percent(20.),
            margin: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        text: Text::from_section(
            "Level 1",
            TextStyle {
                font_size: *UiScale(60.),
                ..default()
            },
        ),
        ..default()
    };

    // Wave Banner

    let wave_banner = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Baseline,
            justify_content: JustifyContent::FlexStart,
            width: Val::Percent(30.),
            height: Val::Percent(50.),
            ..default()
        },
        ..default()
    };

    let wave_bar = NodeBundle {
        style: Style {
            width: Val::Percent(90.),
            height: Val::Percent(2.),
            ..default()
        },
        background_color: BackgroundColor(Color::from(DARK_RED)),
        ..default()
    };

    let wave_text = TextBundle {
        style: Style {
            width: Val::Percent(50.),
            height: Val::Percent(20.),
            margin: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        text: Text::from_section(
            "Wave 1",
            TextStyle {
                font_size: *UiScale(60.),
                ..default()
            },
        ),
        ..default()
    };

    let ui_container_entity = commands
        .spawn(ui_container)
        .insert(StateScoped(Screen::Playing))
        .id();
    let level_bar_entity = commands
        .spawn(level_bar)
        .insert(LevelBar)
        .insert(Name::new("Level Bar"))
        .id();
    let health_bar_entity = commands
        .spawn(health_bar)
        .insert(HealthBar)
        .insert(Name::new("Health Bar"))
        .id();
    let level_text_entity = commands.spawn(level_text).insert(LevelText).id();

    let wave_banner_entity = commands.spawn(wave_banner).id();
    let wave_bar_entity = commands
        .spawn(wave_bar)
        .insert(WaveBar)
        .insert(Name::new("Wave Bar"))
        .id();
    let wave_text_entity = commands.spawn(wave_text).insert(WaveText).id();
    commands
        .entity(wave_banner_entity)
        .push_children(&[wave_bar_entity, wave_text_entity]);

    commands.entity(ui_container_entity).push_children(&[
        level_bar_entity,
        health_bar_entity,
        level_text_entity,
        wave_banner_entity,
    ]);
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

fn update_health_bar(
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    player_health_query: Query<&Damageable, With<Player>>,
) {
    for mut style in &mut health_bar_query {
        if let Ok(player_health) = player_health_query.get_single() {
            let percent_fill = (player_health.health / player_health.max_health) * 100.;
            style.width = Val::Percent(percent_fill);
        };
    }
}

fn update_wave_bar(mut wave_bar_query: Query<&mut Style, With<WaveBar>>, wave: Res<Wave>) {
    for mut style in &mut wave_bar_query {
        style.width = Val::Percent(wave.timer.fraction_remaining() * 100.);
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
