// Handles the logic for a wave of enemies attacking the player

use avian2d::collision::Collider;
use avian2d::prelude::{CollisionLayers, LinearVelocity, LockedAxes, RigidBody};
use bevy::utils::HashMap;
use bevy::{app::App, math::vec3, prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use std::{f32::consts::PI, ops::Neg, time::Duration};

use super::animation::EnemyAnimation;
use super::ItemDrop;
use crate::game::enemy_casting::add_enemy_aim;
use crate::{
    config::*,
    game::{
        assets::{ImageAsset, ImageAssets},
        levelling::Experience,
        physics::GameLayer,
        player_mods::health::HealEvent,
        projectiles::{ProjectileDamage, ProjectileTeam},
        spawn::player::Player,
        Damageable,
    },
    screen::{GameState, Screen},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(clear_wave);
    app.register_type::<Enemy>();
    app.register_type::<EnemyKind>();
    app.init_resource::<Wave>();
    app.init_resource::<WaveState>();
    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(Screen::Playing), reset_wave);
    app.add_systems(
        Update,
        (
            tick_wave,
            spawn_enemies.run_if(
                on_timer(Duration::from_secs_f32(1.)).and_then(resource_equals(WaveState::Active)),
            ),
            chase_player,
            clear_dead_enemies,
        )
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct Wave {
    number: u32,
    spawn_rate_per_sec: u32,
    max_enemies: u32,
    ranged_enemies: u32,
    tank_enemies: u32,
    pub timer: Timer,
}

impl Wave {
    fn new(num: u32, srps: u32, me: u32, re: u32, te: u32, dur: f64) -> Self {
        Wave {
            number: num,
            spawn_rate_per_sec: srps,
            max_enemies: me,
            ranged_enemies: re,
            tank_enemies: te,
            timer: Timer::new(Duration::from_secs_f64(dur), TimerMode::Once),
        }
    }

    fn increment(self) -> Self {
        match self.number {
            // todo tweak numbers to be more balanced
            n @ 1..=5 => Wave::new(n + 1, n * 2, n * 15, n * 2, 1, 45.),
            n @ 6..=10 => Wave::new(n + 1, n * 2, n * 20, n * 3, n * 5, 50.),
            n @ 11..=15 => Wave::new(n + 1, n * 3, n * 25, n * 5, n * 5, 55.),
            n @ 16..=20 => Wave::new(n + 1, n * 3, n * 30, n * 10, n * 8, 60.),
            n @ 21..=25 => Wave::new(n + 1, n * 4, n * 35, n * 10, n * 10, 65.),
            n @ 26..=30 => Wave::new(n + 1, n * 4, n * 40, n * 10, n * 12, 70.),
            n @ 31..=35 => Wave::new(n + 1, n * 5, n * 45, n * 20, n * 14, 75.),
            n @ 36..=40 => Wave::new(n + 1, n * 5, n * 50, n * 20, n * 16, 80.),
            n @ 41..=45 => Wave::new(n + 1, n * 6, n * 55, n * 20, n * 18, 85.),
            n @ 46..=50 => Wave::new(n + 1, n * 6, n * 60, n * 20, n * 20, 90.),
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
            ranged_enemies: 1,
            tank_enemies: 1,
            timer: Timer::new(Duration::from_secs_f64(45.), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct WaveText;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Reflect)]
#[reflect(Component)]
pub enum EnemyKind {
    #[default]
    Basic,
    Ranged {
        proximity: u32,
    },
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
        let xp_modifier = (diff as f32 * 1.02).floor() as u32;
        let dmg_modifier = diff as f32 * 1.125;
        EnemyBundle {
            name: Name::new("Enemy"),
            tag: Enemy,
            breed: EnemyKind::Basic,
            health: Damageable {
                max_health: (ENEMY_HEALTH * hp_modifier).ceil(),
                health: (ENEMY_HEALTH * hp_modifier).ceil(),
                team: ProjectileTeam::Enemy,
                invincibility_timer: Some(Duration::from_secs_f32(0.05)),
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
                damage: (ENEMY_DAMAGE * dmg_modifier).ceil(),
                hits_remaining: 1000,
                knockback_force: 0.4,
            },
        }
    }

    fn ranged(x: f32, y: f32, diff: u32, sprites: &Res<ImageAssets>) -> EnemyBundle {
        let mut ranged = Self::basic(x, y, diff, sprites);
        ranged.breed = EnemyKind::Ranged {
            proximity: RANGED_ENEMY_DIST,
        };
        ranged.health.max_health *= 0.75;
        ranged.sprite.texture = sprites[&ImageAsset::RangedEnemy].clone_weak();
        ranged.collision_box = Collider::circle(16.0);
        ranged.damage.damage *= 1.05;
        ranged.damage.knockback_force = 1.5;
        ranged
    }

    fn tank(x: f32, y: f32, diff: u32, sprites: &Res<ImageAssets>) -> EnemyBundle {
        let mut tank = Self::basic(x, y, diff, sprites);
        tank.breed = EnemyKind::Tank;
        tank.health.max_health *= 1.5;
        // tank.health.invincibility_timer = Some(Duration::from_secs_f32(0.5));
        tank.sprite.texture = sprites[&ImageAsset::TankEnemy].clone_weak();
        tank.collision_box = Collider::ellipse(30.0, 20.0);
        tank.damage.damage *= 0.85;
        tank.damage.knockback_force = 2.5;
        tank
    }
}

#[derive(Resource)]
struct EnemyAtlases {
    map: HashMap<EnemyKind, Handle<TextureAtlasLayout>>,
}

impl EnemyAtlases {
    fn initialise(mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) -> Self {
        let mut enemy_sprites = EnemyAtlases {
            map: HashMap::new(),
        };
        let ranged_layout: TextureAtlasLayout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, Some(UVec2::splat(0)), None);
        let ranged_handle = texture_atlas_layouts.add(ranged_layout);
        enemy_sprites.map.insert(
            EnemyKind::Ranged {
                proximity: RANGED_ENEMY_DIST,
            },
            ranged_handle,
        );

        let tank_layout: TextureAtlasLayout =
            TextureAtlasLayout::from_grid(UVec2::new(64, 48), 4, 1, Some(UVec2::splat(0)), None);
        let tank_handle = texture_atlas_layouts.add(tank_layout);
        enemy_sprites.map.insert(EnemyKind::Tank, tank_handle);
        enemy_sprites
    }
}

#[derive(Bundle)]
struct AnimatedEnemyBundle {
    base: EnemyBundle,
    texture_atlas: TextureAtlas,
    animation: EnemyAnimation,
}

impl AnimatedEnemyBundle {
    fn ranged(
        x: f32,
        y: f32,
        diff: u32,
        sprites: &Res<ImageAssets>,
        enemy_sprites: &EnemyAtlases,
    ) -> Self {
        AnimatedEnemyBundle {
            base: EnemyBundle::ranged(x, y, diff, sprites),
            texture_atlas: TextureAtlas {
                layout: enemy_sprites
                    .map
                    .get(&EnemyKind::Ranged {
                        proximity: RANGED_ENEMY_DIST,
                    })
                    .unwrap()
                    .clone(),
                index: 0,
            },
            animation: EnemyAnimation::new(),
        }
    }

    fn tank(
        x: f32,
        y: f32,
        diff: u32,
        sprites: &Res<ImageAssets>,
        enemy_sprites: &EnemyAtlases,
    ) -> Self {
        AnimatedEnemyBundle {
            base: EnemyBundle::tank(x, y, diff, sprites),
            texture_atlas: TextureAtlas {
                layout: enemy_sprites.map.get(&EnemyKind::Tank).unwrap().clone(),
                index: 0,
            },
            animation: EnemyAnimation::new(),
        }
    }
}

#[derive(Resource, Debug, Default, PartialEq)]
pub enum WaveState {
    #[default]
    Active,
    Inactive,
}

#[derive(Debug, Event)]
pub struct StartWave;

#[derive(Component)]
pub struct ExpireTimer {
    pub timer: Timer,
}

fn reset_wave(mut wave: ResMut<Wave>) {
    *wave = Wave::default();
}

fn tick_wave(
    // _trigger: Trigger<StartWave>,
    mut commands: Commands,
    time: Res<Time>,
    mut curr_wave: ResMut<Wave>,
    mut wave_text_query: Query<&mut Text, With<WaveText>>,
) {
    curr_wave.timer.tick(time.delta());

    if curr_wave.timer.finished() {
        commands.insert_resource(WaveState::Inactive);
        commands.trigger(HealEvent(2.5 + (curr_wave.clone().number as f32 * 1.05)));

        let new_wave = curr_wave.into_inner().clone().increment();
        commands.remove_resource::<Wave>();
        commands.insert_resource(new_wave.clone());

        // commands.trigger(Sfx::WaveComplete);
        let mut text = wave_text_query.single_mut();
        text.sections[0].value = format!("Wave {:?}", new_wave.number);
        commands.trigger(StartWave);
    } else {
        // Continue with current wave
        commands.insert_resource(WaveState::Active);
    }
}

fn setup(mut commands: Commands, texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.insert_resource(EnemyAtlases::initialise(texture_atlas_layouts));
}

fn spawn_enemies(
    mut commands: Commands,
    wave: Res<Wave>,
    wave_state: Res<WaveState>,
    images: Res<ImageAssets>,
    enemy_sprites_map: Res<EnemyAtlases>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let curr_enemies = enemy_query.iter().len() as u32;
    if curr_enemies >= wave.max_enemies
        || player_query.is_empty()
        || wave_state.eq(&WaveState::Inactive)
    {
        return;
    }

    let e_sprites = enemy_sprites_map.as_ref();

    let enemy_spawn_limit: u32 = (wave.max_enemies - curr_enemies).min(wave.spawn_rate_per_sec);
    let player_pos = player_query.single().translation.truncate();

    for n in 0..enemy_spawn_limit {
        let (x, y) = get_random_pos_around(player_pos);
        let ranged_enemy_limit = wave.ranged_enemies;
        let tank_enemy_limit = wave.tank_enemies;
        match n {
            _ if (0..=tank_enemy_limit).contains(&n) => commands.spawn((
                AnimatedEnemyBundle::tank(x, y, wave.number, &images, e_sprites),
            )),
            _ if (tank_enemy_limit + 1..=ranged_enemy_limit).contains(&n) => {
                let re = commands.spawn((AnimatedEnemyBundle::ranged(
                    x,
                    y,
                    wave.number,
                    &images,
                    e_sprites,
                ),));
                let re_id = re.id();
                add_enemy_aim(re_id, &mut commands);

                commands.entity(re_id)
            }
            _ if (ranged_enemy_limit + 1..=enemy_spawn_limit).contains(&n) => {
                commands.spawn(EnemyBundle::basic(x, y, wave.number, &images))
            }
            _ => unreachable!("Enemy ranges not exhaustive at wave: {wave:?}"),
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
    mut enemy_query: Query<
        (
            &mut LinearVelocity,
            &GlobalTransform,
            &EnemyKind,
            &mut Sprite,
        ),
        (With<Enemy>, Without<Player>),
    >,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation();
    for (mut lvelocity, gtransform, enemy_type, mut enemy_sprite) in enemy_query.iter_mut() {
        let player_proximity = (player_pos - gtransform.translation()).length();
        let dir = (player_pos - gtransform.translation()).normalize();

        //check if dir is pointing left:
        enemy_sprite.flip_x = dir.x < 0.0;

        let target_velocity = dir * ENEMY_SPEED;
        match enemy_type {
            EnemyKind::Ranged { proximity } if (*proximity as f32) > player_proximity => {
                //test to print rages:
                // info!("e prox: {}, p prox: {}", proximity, player_proximity);
                lvelocity.0 = lvelocity.0.lerp(target_velocity.neg().xy(), 0.1);
            }
            _ => {
                //lerp velocity towards target velocity
                lvelocity.0 = lvelocity.0.lerp(target_velocity.xy(), 0.1);
            }
        };
    }
}

fn clear_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<
        (&Damageable, &Transform, &Experience, Entity),
        (With<Enemy>, Without<Player>),
    >,
    images: Res<ImageAssets>,
) {
    if enemy_query.is_empty() {
        return;
    }

    // let rng = rand::thread_rng();
    for (health, pos, xp, enemy) in enemy_query.iter() {
        let mut exp_pos = *pos;
        exp_pos.translation.z += 30.;

        if health.health <= 0.0 {
            commands.entity(enemy).despawn_recursive();
            commands.spawn((
                Name::new("Xp drop"),
                *xp,
                ItemDrop,
                SpriteBundle {
                    texture: images[&ImageAsset::Exp].clone_weak(),
                    transform: exp_pos,
                    ..default()
                },
                ExpireTimer {
                    timer: Timer::new(Duration::from_secs(35), TimerMode::Once),
                },
                Collider::circle(1.),
                CollisionLayers::new(GameLayer::Pickups, GameLayer::Player),
                StateScoped(Screen::Playing),
            ));
            // todo xp drops should only live for a short while
        }
    }
}

#[derive(Event, Debug)]
pub struct ClearWave;

fn clear_wave(
    _trigger: Trigger<ClearWave>,
    mut commands: Commands,
    all_enemies: Query<Entity, With<Enemy>>,
) {
    if all_enemies.is_empty() {
        return;
    }

    let _ = all_enemies
        .iter()
        .map(|e| commands.entity(e).despawn_recursive());
}
