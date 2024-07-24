use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Clone, Copy, Debug)]
pub enum GameLayer {
    Border,
    Environment,
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}
