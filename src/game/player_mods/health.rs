use bevy::prelude::*;

use crate::game::spawn::player::Player;
use crate::game::Damageable;

pub(super) fn plugin(app: &mut App) {
    app.observe(heal);
}

#[derive(Event, Debug)]
pub struct HealEvent(pub f32);

fn heal(trigger: Trigger<HealEvent>, mut player_query: Query<&mut Damageable, With<Player>>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.health += trigger.event().0;
        if player.health > player.max_health {
            player.health = player.max_health;
        }
    }
}
