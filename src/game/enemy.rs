// Handles the logic for a wave of enemies attacking the player

use std::f32::consts::PI;
use std::time::Duration;
use rand::Rng;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    {app::App, prelude::Event, math::vec3},
};

use crate::{
    config::*,
    screen::Screen,
    game::assets::{ImageAsset, ImageAssets},
    game::spawn::{player::Player, Health},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(start_wave);
    app.observe(clear_wave);
    app.register_type::<Enemy>();
    app.add_systems(
        Update, 
        (
            spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_PERIOD))),
            chase_player,
            clear_dead_enemies,
        ).run_if(in_state(Screen::Playing))
    );

}

#[derive(Event, Debug)]
pub struct StartWave;

fn start_wave(
    _trigger: Trigger<StartWave>,
) {}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

fn spawn_enemies(
    mut commands: Commands,
    images: Res<ImageAssets>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let curr_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_ENEMIES - curr_enemies).min(SPAWN_RATE_PER_SECOND);

    if curr_enemies >= MAX_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let (x,y) = get_random_pos_around(player_pos);

        commands.spawn((
            Name::new("Enemy"),
            Enemy,
            Health(ENEMY_HEALTH),
            SpriteBundle {
                texture: images[&ImageAsset::Ducky].clone_weak(),
                transform: Transform::from_translation(vec3(x, y, 1.0)),
                ..default()
            },
            StateScoped(Screen::Playing),
        ));
    };
}

fn get_random_pos_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(SPAWN_RADIUS);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

//Enemies will always follow the position of the player
fn chase_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * ENEMY_SPEED;
    }
}

fn clear_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Health, Entity), (With<Enemy>, Without<Player>)>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (health, enemy) in enemy_query.iter() {
        if health.0 <= 0.0 {
            commands.entity(enemy).despawn();
        }
    }
}

#[derive(Event, Debug)]
pub struct ClearWave;

fn clear_wave(
    _trigger: Trigger<ClearWave>,
    mut commands: Commands,
    all_enemies: Query<Entity, With<Enemy>>
) {
    if all_enemies.is_empty() {
        return;
    }

    let _ = all_enemies.iter().map(|e| commands.entity(e).despawn());
}