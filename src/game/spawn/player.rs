//! Spawn the player.

use std::time::Duration;

// use crate::game::lighting::{GameLight, LightMaterial};
use crate::game::physics::GameLayer;
use crate::game::player_mods::damage::player_hit_by_projectile;
use crate::game::player_mods::movement::{Movement, PlayerMovement};
use crate::game::projectiles::ProjectileTeam;
use crate::{
    config::{PLAYER_HEALTH, PLAYER_SPEED},
    game::{
        animation::PlayerAnimation,
        assets::{ImageAsset, ImageAssets},
        levelling::PlayerLevel,
        Damageable,
    },
    screen::Screen,
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // #[allow(dead_code)] _light_materials: ResMut<Assets<LightMaterial>>,
    #[allow(dead_code)] _meshes: ResMut<Assets<Mesh>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 3, Some(UVec2::splat(0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let mut p = commands.spawn((
        Name::new("Wizard"),
        Player,
        Damageable {
            max_health: PLAYER_HEALTH,
            health: PLAYER_HEALTH,
            team: ProjectileTeam::Player,
            invincibility_timer: Some(Duration::from_secs_f32(0.5)),
        },
        PlayerLevel::default(),
        SpriteBundle {
            texture: images[&ImageAsset::Wizard].clone_weak(),
            transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        PlayerMovement::default(),
        Movement {
            speed: PLAYER_SPEED,
        },
        player_animation,
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Dynamic,
        Collider::ellipse(8., 10.),
        CollisionLayers::new(
            GameLayer::Player,
            [
                GameLayer::Border,
                GameLayer::Environment,
                GameLayer::Enemy,
                GameLayer::EnemyProjectile,
                GameLayer::Pickups,
            ],
        ),
        LinearVelocity::default(),
        StateScoped(Screen::Playing),
    ));

    // p.insert(GameLight {
    //     radius: 100.0,
    //     priority: 1,
    // });

    p.observe(player_hit_by_projectile);

    // p.with_children(|parent| {
    //     parent.spawn((
    //         Name::new("Darkness"),
    //         MaterialMesh2dBundle {
    //             mesh: meshes.add(Rectangle::new(660.,380.)).into(),
    //             material: light_materials.add(LightMaterial {
    //                 color: LinearRgba::new(0.0,0.0,0.0,0.9),
    //                 light_count: 2,
    //                 lights: [Vec3::ZERO; 64],
    //             }),
    //             transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
    //             ..Default::default()
    //         },
    //         StateScoped(Screen::Playing),
    //     ));
    // });
}
