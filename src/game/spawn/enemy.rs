use std::f32::consts::PI;
use rand::Rng;
use bevy::{math::vec3, prelude::*};

use crate::{
    game::assets::{ImageAsset, ImageAssets},
    config::*,
};

use super::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_enemies);
    app.observe(clear_enemies);
    app.register_type::<Enemy>();
}

#[derive(Event, Debug)]
pub struct SpawnEnemies;

#[derive(Event, Debug)]
pub struct ClearEnemies;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

fn spawn_enemies(
    _trigger: Trigger<SpawnEnemies>,
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
        let (xp,y) = get_random_pos_around(player_pos);

        commands.spawn((
            SpriteBundle {
                texture: images[&ImageAsset::Ducky].clone_weak(),
                transform: Transform::from_translation(vec3(xp, y, 1.0)),
                ..default()
            },
            Enemy
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

fn clear_enemies(
    _trigger: Trigger<ClearEnemies>,
    mut commands: Commands,
    all_enemies: Query<Entity, With<Enemy>>
) {
    if all_enemies.is_empty() {
        return;
    }

    let _ = all_enemies.iter().map(|e| commands.entity(e).despawn());
}