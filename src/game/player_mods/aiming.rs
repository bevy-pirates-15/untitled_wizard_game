use avian2d::math::Quaternion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::spawn::player::Player;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record where player aims
    app.register_type::<PlayerAim>();
    // Apply wand aim
    app.add_systems(
        Update,
        (
            player_mouse_look.in_set(AppSet::RecordInput),
            attach_to_player.in_set(AppSet::Update),
        ),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerAim(pub Vec2);

// fn wand_aiming(
//     mut aim_query: Query<(&PlayerAim, &mut Transform)>,
//     player_query: Query<&Transform, (With<Player>, Without<PlayerAim>)>,
// ) {
//     let Ok(player_transform) = player_query.get_single() else {
//         return;
//     };
//     for (player_aim, mut transform) in &mut aim_query {
//         if player_aim.0.length_squared() < 0.01 {
//             continue;
//         }
//
//         let angle = -player_aim.0.x.atan2(player_aim.0.y);
//
//         transform.translation = player_transform.translation + Vec3::new(0.0, 0.0, 1.0);
//         transform.rotation = Quaternion::from_rotation_z(angle);
//     }
// }

pub fn player_mouse_look(
    mut aim_query: Query<(&mut PlayerAim, &mut Transform, &GlobalTransform)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let window = window_query
        .get_single()
        .expect("Need a single primary window");

    // Many steps can fail here, so we'll wrap in an option pipeline
    // First check if the cursor is in window
    // Then check if the ray intersects the plane defined by the player
    // Then finally compute the point along the ray to look at
    if let Some(p) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| {
            Some(ray).zip(ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        for (mut aim, mut transform, gtransform) in aim_query.iter_mut() {
            let diff = (p - gtransform.translation()).xy();

            if diff.length_squared() > 0.01 {
                aim.0 = diff;
                let angle = -diff.x.atan2(diff.y);

                transform.translation = gtransform.translation() + Vec3::new(0.0, 0.0, 1.0);
                transform.rotation = Quaternion::from_rotation_z(angle);
            }
        }
    }

    // let Ok(player_transform) = player_query.get_single() else {
    //     return;
    // };
    // for (player_aim, mut transform) in &mut aim_query {
    //     if player_aim.0.length_squared() < 0.01 {
    //         continue;
    //     }
    //
    //     let angle = -player_aim.0.x.atan2(player_aim.0.y);
    //
    //     transform.translation = player_transform.translation + Vec3::new(0.0, 0.0, 1.0);
    //     transform.rotation = Quaternion::from_rotation_z(angle);
    // }
}

#[derive(Component, Debug, Clone)]
pub struct AttachToPlayer;
fn attach_to_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut query: Query<(&AttachToPlayer, &mut Transform)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (_, mut transform) in query.iter_mut() {
        transform.translation = player_transform.translation();
    }
}
