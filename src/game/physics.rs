use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screen::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Running), unpause);
    app.add_systems(OnExit(GameState::Running), pause);
}

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum GameLayer {
    Border,
    Environment,
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
