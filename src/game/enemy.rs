// Handles the logic for a wave of enemies attacking the player

use avian2d::collision::Collider;
use avian2d::prelude::CollisionLayers;
use bevy::{
    app::App,
    color::palettes::css::LIGHT_CORAL,
    math::vec3,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::common_conditions::on_timer,
};
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

use super::ItemDrop;
use crate::game::physics::GameLayer;
use crate::{
    config::*,
    game::{
        assets::{ImageAsset, ImageAssets},
        levelling::Experience,
        spawn::player::Player,
        Health,
    },
    screen::{GameState, Screen},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(start_wave);
    app.register_type::<Enemy>();
    app.init_resource::<Wave>();
    app.init_resource::<WaveState>();
    app.add_systems(
        Update,
        (
            spawn_enemies.run_if(
                on_timer(Duration::from_secs_f32(1.)).and_then(resource_equals(WaveState::Active)),
            ),
            chase_player,
            clear_dead_enemies,
        )
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Resource, Debug, Clone)]
struct Wave {
    number: u32,
    spawn_period: f32,
    spawn_rate_per_sec: usize,
    max_enemies: usize,
    timer: Timer,
}

impl Wave {
    fn new(num: u32, sp: f32, srps: usize, me: usize, dur: f64) -> Self {
        Wave {
            number: num,
            spawn_period: sp,
            spawn_rate_per_sec: srps,
            max_enemies: me,
            timer: Timer::new(Duration::from_secs_f64(dur), TimerMode::Once),
        }
    }

    fn increment(self) -> Self {
        match self.number {
            // todo tweak numbers to be more balanced
            n @ 1..=5 => Wave::new(n + 1, 10., n as usize * 2, n as usize * 15, 45.),
            n @ 6..=10 => Wave::new(n + 1, 9., n as usize * 2, n as usize * 20, 50.),
            n @ 11..=15 => Wave::new(n + 1, 8., n as usize * 3, n as usize * 25, 55.),
            n @ 16..=20 => Wave::new(n + 1, 7., n as usize * 3, n as usize * 30, 60.),
            n @ 21..=25 => Wave::new(n + 1, 6., n as usize * 4, n as usize * 35, 65.),
            n @ 26..=30 => Wave::new(n + 1, 5., n as usize * 4, n as usize * 40, 70.),
            n @ 31..=35 => Wave::new(n + 1, 4., n as usize * 5, n as usize * 45, 75.),
            n @ 36..=40 => Wave::new(n + 1, 3., n as usize * 5, n as usize * 50, 80.),
            n @ 41..=45 => Wave::new(n + 1, 2., n as usize * 6, n as usize * 55, 85.),
            n @ 46..=50 => Wave::new(n + 1, 1., n as usize * 6, n as usize * 60, 90.),
            51..=u32::MAX => self,
            _ => unreachable!("Wave number out of bounds!"),
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Wave {
            number: 1,
            spawn_period: 10.,
            spawn_rate_per_sec: 10,
            max_enemies: 30,
            timer: Timer::new(Duration::from_secs_f64(30.), TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Resource, Debug, Default, PartialEq)]
enum WaveState {
    Active,
    #[default]
    Inactive,
}

#[derive(Debug, Event)]
pub struct StartWave;

fn start_wave(
    _trigger: Trigger<StartWave>,
    mut commands: Commands,
    time: Res<Time>,
    mut curr_wave: ResMut<Wave>,
) {
    curr_wave.timer.tick(time.delta());

    if curr_wave.timer.finished() {
        commands.insert_resource(WaveState::Inactive);
        let new_wave = curr_wave.into_inner().clone().increment();
        commands.remove_resource::<Wave>();
        commands.insert_resource(new_wave);
        commands.trigger(StartWave);
    } else {
        // Continue with current wave
        commands.insert_resource(WaveState::Active);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    wave: Res<Wave>,
    images: Res<ImageAssets>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let curr_enemies = enemy_query.iter().len();
    if curr_enemies >= wave.max_enemies || player_query.is_empty() {
        return;
    }

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_pos = player_query.single().translation.truncate();
    let enemy_spawn_count = (wave.max_enemies - curr_enemies).min(wave.spawn_rate_per_sec);
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_pos_around(player_pos);

        commands.spawn((
            Name::new("Enemy"),
            Enemy,
            Health(ENEMY_HEALTH * wave.number as f32),
            Experience(BASE_ENEMY_XP),
            SpriteBundle {
                texture: images[&ImageAsset::Ducky].clone_weak(),
                transform: Transform::from_translation(vec3(x, y, 2.0)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 1,
            },
            StateScoped(Screen::Playing),
            Collider::circle(8.),
            CollisionLayers::new(
                GameLayer::Enemy,
                [
                    GameLayer::Environment,
                    GameLayer::Player,
                    GameLayer::PlayerProjectile,
                ],
            ),
        ));
    }
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
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
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
    enemy_query: Query<(&Health, &Transform, &Experience, Entity), (With<Enemy>, Without<Player>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (health, pos, xp, enemy) in enemy_query.iter() {
        if health.0 <= 0.0 {
            commands.entity(enemy).despawn();
            commands.spawn((
                Name::new("Xp drop"),
                *xp,
                ItemDrop,
                MaterialMesh2dBundle {
                    //todo add texture
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., 20.))),
                    material: materials.add(Color::from(LIGHT_CORAL)),
                    transform: *pos,
                    ..default()
                },
                Collider::circle(20.),
            ));
            // todo xp drops should only live for a short while
        }
    }
}
