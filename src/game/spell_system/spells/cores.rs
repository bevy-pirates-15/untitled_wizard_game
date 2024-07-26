use crate::game::assets::spell_gfx::SpellGFXAsset;
use crate::game::projectiles::ProjectileDamage;
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::helpers::{spawn_spell_projectile, ProjectileStats, SpellModel};
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect};
use bevy::color::Color;
use bevy::log::{info, warn};
use bevy::math::Vec2;
use bevy::prelude::{Circle, MeshBuilder, Meshable, World};
use bevy::sprite::ColorMaterial;
use std::f32::consts::PI;
use std::slice::Iter;
use std::sync::Arc;
use std::time::Duration;

pub(super) fn get_spells() -> Vec<SpellComponent> {
    vec![
        SpellComponent {
            data: Box::new(ZapSpellData { base_damage: 40.0 }),
            icon_id: 0,
        },
        SpellComponent {
            data: Box::new(BangSpellData {
                base_damage: 40.0,
                radius: 100.0,
            }),
            icon_id: 3,
        },
        SpellComponent {
            data: Box::new(ArcaneArrowSpellData {
                base_damage: 30.0,
                speed: 500.0,
                num_hits: 3,
            }),
            icon_id: 1,
        },
        SpellComponent {
            data: Box::new(SplitterBoltsSpellData {
                base_damage: 20.0,
                projectile_count: 3,
            }),
            icon_id: 2,
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
            SpellModel::StaticSprite(SpellGFXAsset::Zap),
            ProjectileStats {
                radius: 5.,
                speed: 500.0,
                damage: self.base_damage,
                num_hits: 1,
                lifetime: Duration::from_secs_f32(2.0),
                knockback_force: 200.0,
            },
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
            SpellModel::MeshMat(
                Circle {
                    radius: self.radius,
                }
                .mesh()
                .build(),
                ColorMaterial::from(Color::WHITE),
            ),
            ProjectileStats {
                radius: self.radius,
                speed: 0.0,
                damage: self.base_damage,
                num_hits: 1000,
                lifetime: Duration::from_secs_f32(0.05),
                knockback_force: 100.0,
            }, // self.radius,
               // 0.0,
               // self.base_damage,
               // 1000,
               // Duration::from_secs_f32(0.05),
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
            SpellModel::StaticSprite(SpellGFXAsset::ArcaneArrow),
            ProjectileStats {
                radius: 5.,
                speed: self.speed,
                damage: self.base_damage,
                num_hits: self.num_hits,
                lifetime: Duration::from_secs_f32(2.0),
                knockback_force: 100.0,
            },
        ) else {
            warn!("Failed to spawn arcane arrow spell entity");
            return;
        };
        let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
        info!("Cast Arcane Arrow - DMG: {}", spell_damage);
    }
}

////////////////////
// splitter BOLTS //
////////////////////
// A simple spell that fires X small projectiles.

#[derive(Clone)]
pub struct SplitterBoltsSpellData {
    pub base_damage: f32,
    pub projectile_count: u32,
}
impl SpellData for SplitterBoltsSpellData {
    fn build(&self, _iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        Some(Arc::new(SplitterBoltsSpell {
            base_damage: self.base_damage,
            projectile_count: self.projectile_count,
        }))
    }

    fn get_name(&self) -> String {
        String::from("splitter Bolts Spell")
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
pub struct SplitterBoltsSpell {
    pub base_damage: f32,
    pub projectile_count: u32,
}
impl SpellEffect for SplitterBoltsSpell {
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
                SpellModel::StaticSprite(SpellGFXAsset::SplitterBolts),
                ProjectileStats {
                    radius: 5.,
                    speed: 250.0,
                    damage: self.base_damage,
                    num_hits: 1,
                    lifetime: Duration::from_secs_f32(2.0),
                    knockback_force: 50.0,
                },
            ) else {
                warn!("Failed to spawn splitter bolts spell entity");
                return;
            };
            let spell_damage = world.get::<ProjectileDamage>(spell_entity).unwrap().damage;
            info!("Cast splitter Bolts - DMG: {}", spell_damage);
        }
    }
}
