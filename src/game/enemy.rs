// Handles the logic for a wave of enemies attacking the player

use avian2d::collision::Collider;
use avian2d::prelude::{CollisionLayers, LinearVelocity, LockedAxes, RigidBody};
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
use crate::{
    config::*,
    game::{
        assets::{ImageAsset, ImageAssets},
        levelling::Experience,
        spawn::player::Player,
        Damageable,
        physics::GameLayer,
        projectiles::{ProjectileDamage, ProjectileTeam},
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
    spawn_rate_per_sec: usize,
    max_enemies: usize,
    timer: Timer,
}

impl Wave {
    fn new(num: u32, srps: usize, me: usize, dur: f64) -> Self {
        Wave {
            number: num,
            spawn_rate_per_sec: srps,
            max_enemies: me,
            timer: Timer::new(Duration::from_secs_f64(dur), TimerMode::Once),
        }
    }

    fn increment(self) -> Self {
        match self.number {
            // todo tweak numbers to be more balanced
            n @ 1..=5 => Wave::new(n + 1, n as usize * 2, n as usize * 15, 45.),
            n @ 6..=10 => Wave::new(n + 1, n as usize * 2, n as usize * 20, 50.),
            n @ 11..=15 => Wave::new(n + 1,  n as usize * 3, n as usize * 25, 55.),
            n @ 16..=20 => Wave::new(n + 1,  n as usize * 3, n as usize * 30, 60.),
            n @ 21..=25 => Wave::new(n + 1,  n as usize * 4, n as usize * 35, 65.),
            n @ 26..=30 => Wave::new(n + 1,  n as usize * 4, n as usize * 40, 70.),
            n @ 31..=35 => Wave::new(n + 1,  n as usize * 5, n as usize * 45, 75.),
            n @ 36..=40 => Wave::new(n + 1,  n as usize * 5, n as usize * 50, 80.),
            n @ 41..=45 => Wave::new(n + 1,  n as usize * 6, n as usize * 55, 85.),
            n @ 46..=50 => Wave::new(n + 1,  n as usize * 6, n as usize * 60, 90.),
            51..=u32::MAX => self,
            _ => unreachable!("Wave number out of bounds!"),
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Wave {
            number: 1,
            spawn_rate_per_sec: 10,
            max_enemies: 30,
            timer: Timer::new(Duration::from_secs_f64(30.), TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub enum EnemyKind {
    #[default]
    Basic,
    Ranged,
    Tank,
}

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    tag: Enemy,
    breed: EnemyKind,
    health: Damageable,
    xp: Experience,
    sprite: SpriteBundle,
    state: StateScoped<Screen>,
    collision_box: Collider,
    collision_layers: CollisionLayers,
    locked_axes: LockedAxes,
    rigid_body: RigidBody,
    linear_velocity: LinearVelocity,
    damage: ProjectileDamage,

}

impl EnemyBundle {
    fn basic(x: f32, y: f32, diff: u32, sprites: &Res<ImageAssets>) -> Self {
        let hp_modifier = diff as f32 * 1.1;
        let xp_modifier = (diff as f32 * 0.2).floor() as u32;
        let dmg_modifier = diff as f32 * 1.125;
        EnemyBundle {
            name: Name::new("Enemy"),
            tag: Enemy,
            breed: EnemyKind::Basic,
            health: Damageable {
                max_health: (ENEMY_HEALTH * hp_modifier).ceil(),
                health: (ENEMY_HEALTH * hp_modifier).ceil(),
                team: ProjectileTeam::Enemy,
                invincibility_timer: Duration::from_secs_f32(0.1),
            },
            xp: Experience(BASE_ENEMY_XP * xp_modifier),
            sprite: SpriteBundle {
                texture: sprites[&ImageAsset::BasicEnemy].clone_weak(),
                transform: Transform::from_translation(vec3(x, y, 2.0)),
                ..default()
            },
            state: StateScoped(Screen::Playing),
            collision_box: Collider::circle(8.),
            collision_layers: CollisionLayers::new(
                GameLayer::Enemy,
                [
                    GameLayer::Enemy,
                    GameLayer::Environment,
                    GameLayer::Player,
                    GameLayer::PlayerProjectile,
                ],
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            rigid_body: RigidBody::Dynamic,
            linear_velocity: LinearVelocity::default(),
            damage: ProjectileDamage {
                team: ProjectileTeam::Enemy,
                damage: (1.0 * dmg_modifier).ceil(),
                hits_remaining: 1000,
                knockback_force: 0.4,
            },
        }
    }

    fn ranged(x:f32, y: f32, diff: u32, sprites: &Res<ImageAssets>) -> EnemyBundle {
        let mut ranged = Self::basic(x, y, diff, sprites);
        ranged.breed = EnemyKind::Ranged;
        ranged.health.max_health *= 0.75;
        ranged.sprite.texture = sprites[&ImageAsset::RangedEnemy].clone_weak();
        ranged.damage.damage *= 1.05;
        ranged.damage.knockback_force = 1.5;
        ranged
    }

    fn tank(x:f32, y: f32, diff: u32, sprites: &Res<ImageAssets>) -> EnemyBundle {
        let mut tank = Self::basic(x, y, diff, sprites);
        tank.breed = EnemyKind::Tank;
        tank.health.max_health *= 1.5;
        tank.health.invincibility_timer = Some(Duration::from_secs_f32(0.5));
        tank.sprite.texture = sprites[&ImageAsset::TankEnemy].clone_weak();
        tank.damage.damage *= 0.85;
        tank.damage.knockback_force = 2.5;
        tank
    }
}

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
    wave_state: Res<WaveState>,
    images: Res<ImageAssets>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let curr_enemies = enemy_query.iter().len();
    if curr_enemies >= wave.max_enemies || player_query.is_empty() || wave_state.eq(&WaveState::Inactive) {
        return;
    }
    let enemy_spawn_limit = (wave.max_enemies - curr_enemies).min(wave.spawn_rate_per_sec);

    //todo when make wave, randomly choose enemy amounts

    let player_pos = player_query.single().translation.truncate();
    for n in 0..enemy_spawn_limit {
        let (x, y) = get_random_pos_around(player_pos);
        let base_enemy_limit = 10;
        let tank_enemy_limit = 20;
        match n {
            _ if (0..=base_enemy_limit-1).contains(&n) => commands.spawn(EnemyBundle::basic(x, y, 10, &images)),
            _ if (base_enemy_limit..=tank_enemy_limit-1).contains(&n) => commands.spawn(EnemyBundle::tank(x, y, 10, &images)),
            _ if (tank_enemy_limit..=enemy_spawn_limit).contains(&n) => commands.spawn(EnemyBundle::ranged(x, y, 10, &images)),
            _ => unreachable!("Enemy ranges not exhaustive at wave: "),
        };
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
pub fn chase_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut enemy_query: Query<(&mut LinearVelocity, &GlobalTransform), (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation();
    for (mut lvelocity, gtransform) in enemy_query.iter_mut() {
        let dir = (player_pos - gtransform.translation()).normalize();
        let target_velocity = dir * ENEMY_SPEED;
        //lerp velocity towards target velocity
        lvelocity.0 = lvelocity.0.lerp(target_velocity.xy(), 0.1);
    }
}

fn clear_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<
        (&Damageable, &Transform, &Experience, Entity),
        (With<Enemy>, Without<Player>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (health, pos, xp, enemy) in enemy_query.iter() {
        if health.health <= 0.0 {
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
                StateScoped(Screen::Playing),
            ));
            // todo xp drops should only live for a short while
        }
    }
}
