use bevy::prelude::{Component, Vec2};

pub enum ProjectileTeam {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct ProjectileStats {
    pub team: ProjectileTeam,
    pub damage: f32,
}

pub enum ProjectileColliderType {
    Circle {
        radius: f32,
    },
    Capsule {
        radius: f32,
        length: f32,
        rotation_offset: f32,
    }, //use this for beams/lasers etc
}
#[derive(Component)]
pub struct ProjectileCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct ProjectileMotion {
    // pub direction: Vec2, //todo: use projectiles transform
    pub speed: f32,
}
