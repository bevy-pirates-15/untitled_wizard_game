use crate::game::projectiles::ProjectileDamage;
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::helpers::spawn_spell_projectile;
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect};
use bevy::log::{info, warn};
use bevy::math::Vec2;
use bevy::prelude::World;
use std::f32::consts::PI;
use std::slice::Iter;
use std::sync::Arc;
use std::time::Duration;

pub(super) fn get_spells() -> Vec<SpellComponent> {
    vec![
        SpellComponent {
            data: Box::new(ZapSpellData { base_damage: 40.0 }),
        },
        SpellComponent {
            data: Box::new(BangSpellData {
                base_damage: 40.0,
                radius: 100.0,
            }),
        },
        SpellComponent {
            data: Box::new(ArcaneArrowSpellData {
                base_damage: 30.0,
                speed: 1000.0,
                num_hits: 3,
            }),
        },
        SpellComponent {
            data: Box::new(SpitterBoltsSpellData {
                base_damage: 20.0,
                projectile_count: 3,
            }),
        },
    ]
}

/////////
// ZAP //
/////////
// A simple spell that fires a jolt of energy dealing damage.

#[derive(Clone)]
pub struct ZapSpellData {
    pub base_damage: f32,
}
impl SpellData for ZapSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(ZapSpell {
            base_damage: self.base_damage,
        }))
    }

    fn get_name(&self) -> String {
        String::from("Zap Spell")
    }

    fn get_desc(&self) -> String {
        String::from("Fires a jolt of energy dealing: ")
            + &self.base_damage.to_string()
            + " damage."
    }
}

#[derive(Debug, Clone)]
pub struct ZapSpell {
    pub base_damage: f32,
}
impl SpellEffect for ZapSpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(spell_entity) = spawn_spell_projectile(
            context,
            world,
            10.,
            500.0,
            self.base_damage,
            1,
            Duration::from_secs_f32(1.0),
        ) else {
            warn!("Failed to spawn zap spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Zap - DMG: {}", spell_damage);
    }
}

//////////
// BANG //
//////////
// A simple spell that explodes dealing damage in an area around the caster.

#[derive(Clone)]
pub struct BangSpellData {
    pub base_damage: f32,
    pub radius: f32,
}
impl SpellData for BangSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(BangSpell {
            base_damage: self.base_damage,
            radius: self.radius,
        }))
    }

    fn get_name(&self) -> String {
        String::from("Bang Spell")
    }

    fn get_desc(&self) -> String {
        String::from("Explodes dealing: ")
            + &self.base_damage.to_string()
            + " damage in a "
            + &self.radius.to_string()
            + " radius."
    }
}

#[derive(Debug, Clone)]
pub struct BangSpell {
    pub base_damage: f32,
    pub radius: f32,
}
impl SpellEffect for BangSpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(spell_entity) = spawn_spell_projectile(
            context,
            world,
            self.radius,
            0.0,
            self.base_damage,
            1000,
            Duration::from_secs_f32(0.05),
        ) else {
            warn!("Failed to spawn bang spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Bang - DMG: {}", spell_damage);
    }
}

//////////////////
// Arcane Arrow //
//////////////////
// A simple spell that fires a projectile that deals damage and pierces through enemies.

#[derive(Clone)]
pub struct ArcaneArrowSpellData {
    pub base_damage: f32,
    pub speed: f32,
    pub num_hits: i32,
}
impl SpellData for ArcaneArrowSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(ArcaneArrowSpell {
            base_damage: self.base_damage,
            speed: self.speed,
            num_hits: self.num_hits,
        }))
    }

    fn get_name(&self) -> String {
        String::from("Arcane Arrow Spell")
    }

    fn get_desc(&self) -> String {
        String::from("Fires an arrow dealing: ")
            + &self.base_damage.to_string()
            + " damage and pierces through: "
            + &self.num_hits.to_string()
            + " enemies."
    }
}

#[derive(Debug, Clone)]
pub struct ArcaneArrowSpell {
    pub base_damage: f32,
    pub speed: f32,
    pub num_hits: i32,
}
impl SpellEffect for ArcaneArrowSpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let Some(spell_entity) = spawn_spell_projectile(
            context,
            world,
            10.,
            self.speed,
            self.base_damage,
            self.num_hits,
            Duration::from_secs_f32(2.0),
        ) else {
            warn!("Failed to spawn arcane arrow spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Arcane Arrow - DMG: {}", spell_damage);
    }
}

////////////////////
// SPITTER BOLTS //
////////////////////
// A simple spell that fires X small projectiles.

#[derive(Clone)]
pub struct SpitterBoltsSpellData {
    pub base_damage: f32,
    pub projectile_count: u32,
}
impl SpellData for SpitterBoltsSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(SpitterBoltsSpell {
            base_damage: self.base_damage,
            projectile_count: self.projectile_count,
        }))
    }

    fn get_name(&self) -> String {
        String::from("Spitter Bolts Spell")
    }

    fn get_desc(&self) -> String {
        String::from("Fires: ")
            + &self.projectile_count.to_string()
            + " bolts dealing: "
            + &self.base_damage.to_string()
            + " damage each."
    }
}

#[derive(Debug, Clone)]
pub struct SpitterBoltsSpell {
    pub base_damage: f32,
    pub projectile_count: u32,
}
impl SpellEffect for SpitterBoltsSpell {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        for _ in 0..self.projectile_count {
            let mut cast_context = context.clone();

            //randomly offset the direction
            cast_context.direction =
                Vec2::from_angle(rand::random::<f32>() * (PI / 4.) - (PI / 8.))
                    .rotate(cast_context.direction);

            let Some(spell_entity) = spawn_spell_projectile(
                &mut cast_context,
                world,
                5.,
                1000.0,
                self.base_damage,
                1,
                Duration::from_secs_f32(2.0),
            ) else {
                warn!("Failed to spawn spitter bolts spell entity");
                return;
            };
            let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
            info!("Cast Spitter Bolts - DMG: {}", spell_damage);
        }
    }
}
