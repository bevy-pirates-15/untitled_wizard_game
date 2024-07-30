/////////
// ENEMY //
/////////
// A simple spell that fires a jolt of energy dealing damage.

use crate::game::assets::particles::ParticleAsset;
use crate::game::assets::spell_gfx::SpellGFXAsset;
use crate::game::projectiles::{ProjectileDamage, ProjectileTeam};
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::helpers::{spawn_spell_projectile, ProjectileStats, SpellModel};
use crate::game::spell_system::SpellEffect;
use bevy::log::{info, warn};
use bevy::prelude::World;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EnemySpell;
impl SpellEffect for EnemySpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(spell_entity) = spawn_spell_projectile(
            context,
            world,
            ProjectileTeam::Enemy,
            SpellModel::StaticSprite(SpellGFXAsset::Enemy),
            Some(ParticleAsset::Enemy),
            ProjectileStats {
                radius: 5.,
                speed: 100.0,
                damage: 1.,
                num_hits: 1,
                lifetime: Duration::from_secs_f32(5.0),
                knockback_force: 200.0,
            },
        ) else {
            warn!("Failed to spawn enemy spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Enemy - DMG: {}", spell_damage);
    }
}
