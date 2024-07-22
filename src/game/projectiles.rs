use bevy::prelude::Component;

pub enum ProjectileTeam {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct ProjectileDamage {
    pub team: ProjectileTeam,
    pub damage: f32,
}
