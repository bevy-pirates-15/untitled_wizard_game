use bevy::prelude::*;

use crate::{
    game::assets::{ImageAsset, ImageAssets},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_temp_prompt);
    app.insert_resource(CountdownTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    app.add_systems(OnEnter(Screen::Playing), reset_count_down);
    app.add_systems(Update, count_down.run_if(in_state(Screen::Playing)));
}

#[derive(Resource)]
struct CountdownTimer(Timer);

#[derive(Event, Debug)]
pub struct SpawnPrompt;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct ToBeDespawned;

fn spawn_temp_prompt(
    _trigger: Trigger<SpawnPrompt>,
    mut commands: Commands,
    images: Res<ImageAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: images[&ImageAsset::MovePrompt].clone_weak(),
            transform: Transform::from_translation(Vec3::new(-100., 0., 20.)),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
        ToBeDespawned,
    ));

    commands.spawn((
        SpriteBundle {
            texture: images[&ImageAsset::ShootPrompt].clone_weak(),
            transform: Transform::from_translation(Vec3::new(100., 0., 20.)),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
        ToBeDespawned,
    ));
}

fn count_down(
    time: Res<Time>,
    mut commands: Commands,
    mut countdown_timer: ResMut<CountdownTimer>,
    prompt_query: Query<Entity, With<ToBeDespawned>>,
) {
    countdown_timer.0.tick(time.delta());
    if countdown_timer.0.finished() {
        for prompt in prompt_query.iter() {
            commands.entity(prompt).despawn_recursive();
        }
    }
}

fn reset_count_down(mut countdown_timer: ResMut<CountdownTimer>) {
    countdown_timer.0.reset();
}
