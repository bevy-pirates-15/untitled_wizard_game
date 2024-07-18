//! Camera logic

use bevy::prelude::*;

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, camera_follow_player);
}

fn camera_follow_player(
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let Ok((mut camera_transform, _camera)) = camera.get_single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
