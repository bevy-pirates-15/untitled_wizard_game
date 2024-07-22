use crate::AppSet;
use avian2d::math::Quaternion;
use bevy::prelude::*;

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    // Record where player aims
    app.register_type::<PlayerAim>();
    // Apply wand aim
    app.add_systems(Update, wand_aiming.in_set(AppSet::RecordInput));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerAim(pub Vec2);

fn wand_aiming(
    mut aim_query: Query<(&PlayerAim, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerAim>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (player_aim, mut transform) in &mut aim_query {
        if player_aim.0.length_squared() < 0.01 {
            continue;
        }

        let angle = -player_aim.0.x.atan2(-player_aim.0.y);

        transform.translation = player_transform.translation + Vec3::new(0.0, 0.0, 1.0);
        transform.rotation = Quaternion::from_rotation_z(angle);
    }
}
