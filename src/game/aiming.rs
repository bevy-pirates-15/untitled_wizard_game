use crate::AppSet;
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
        let player_translation = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        // How far away the wand is from player
        let radius = 100.;
        transform.translation = (player_translation + radius * player_aim.0).extend(3.0);
    }
}
