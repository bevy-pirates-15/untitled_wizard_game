use crate::game::projectiles::{ProjectileDamage, ProjectileLifetime};
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect, SpellModifier};
use avian2d::prelude::LinearVelocity;
use bevy::log::info;
use bevy::prelude::{Entity, World};
use std::slice::Iter;
use std::sync::Arc;
use std::time::Duration;

pub(super) fn get_spells() -> Vec<(SpellComponent, i32)> {
    vec![
        (
            SpellComponent {
                data: Box::new(DmgUpSpellModData {
                    damage_increase: 1.5,
                }),
                icon_id: 12,
            },
            5,
        ),
        (
            SpellComponent {
                data: Box::new(PiercingData {
                    pierce_increase: 1,
                    speed_increase: 1.25,
                }),
                icon_id: 15,
            },
            5,
        ),
        (
            SpellComponent {
                data: Box::new(LifetimeData {
                    lifetime_increase: 1.5,
                }),
                icon_id: 16,
            },
            5,
        ),
        (
            SpellComponent {
                data: Box::new(DuplicateData {
                    spread_increase: 20.,
                    bullet_count: 2,
                    damage_decrease: 0.66,
                }),
                icon_id: 17,
            },
            10,
        ),
        (
            SpellComponent {
                data: Box::new(DuplicateData {
                    spread_increase: 40.,
                    bullet_count: 3,
                    damage_decrease: 0.5,
                }),
                icon_id: 18,
            },
            20,
        ),
    ]
}

////////////
// DMG UP //
////////////
// A simple spell modifier that increases the damage of the spells it contains

#[derive(Clone)]
pub struct DmgUpSpellModData {
    pub damage_increase: f32,
}
impl SpellData for DmgUpSpellModData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let spell = iter.next()?.data.build(iter)?;
        Some(Arc::new(DmgUpSpellMod {
            damage_increase: self.damage_increase,
            spell,
        }))
    }

    fn get_name(&self) -> String {
        "DMG UP".to_string()
    }

    fn get_desc(&self) -> String {
        "The next spells gains: \n".to_string()
            + "- Damage: +"
            + &*(100. * (&self.damage_increase - 1.)).to_string()
            + "%"
    }
}

#[derive(Debug, Clone)]
pub struct DmgUpSpellMod {
    pub damage_increase: f32,
    pub spell: Arc<dyn SpellEffect>,
}
impl SpellEffect for DmgUpSpellMod {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let damage_increase = self.damage_increase;
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            // get ProjectileDamage component
            if let Some(mut projectile_damage) = mod_world.get_mut::<ProjectileDamage>(e) {
                projectile_damage.damage *= damage_increase;
            };
        });

        context.add_modifier("DMGUP Mod", modifier);
        info!("Cast DMGUP Mod");
        self.spell.cast(context, world);
    }
}

#[derive(Clone)]
pub struct PiercingData {
    pub pierce_increase: i32,
    pub speed_increase: f32,
}
impl SpellData for PiercingData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let spell = iter.next()?.data.build(iter)?;
        Some(Arc::new(Piercing {
            pierce_increase: self.pierce_increase,
            speed_increase: self.speed_increase,
            spell,
        }))
    }

    fn get_name(&self) -> String {
        "Penetration".to_string()
    }

    fn get_desc(&self) -> String {
        "The next spells gains: \n".to_string()
            + "- Pierce: +"
            + &self.pierce_increase.to_string()
            + "\n"
            + "- Speed: +"
            + &*(100. * (&self.speed_increase - 1.)).to_string()
            + "%"
    }
}

#[derive(Debug, Clone)]
pub struct Piercing {
    pub pierce_increase: i32,
    pub speed_increase: f32,
    pub spell: Arc<dyn SpellEffect>,
}
impl SpellEffect for Piercing {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let pierce_increase = self.pierce_increase;
        let speed_increase = self.speed_increase;
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            // get ProjectileSpeed component
            if let Some(mut projectile_speed) = mod_world.get_mut::<LinearVelocity>(e) {
                projectile_speed.0 *= speed_increase;
            };
            // get ProjectileDamage component
            if let Some(mut projectile_damage) = mod_world.get_mut::<ProjectileDamage>(e) {
                projectile_damage.hits_remaining += pierce_increase;
            };
        });

        context.add_modifier("Penetration Mod", modifier);
        info!("Cast Piercing Mod");
        self.spell.cast(context, world);
    }
}

#[derive(Clone)]
pub struct DuplicateData {
    pub spread_increase: f32,
    pub bullet_count: i32,
    pub damage_decrease: f32,
}
impl SpellData for DuplicateData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let spell = iter.next()?.data.build(iter)?;
        Some(Arc::new(Duplicate {
            spread_increase: self.spread_increase,
            bullet_count: self.bullet_count,
            damage_decrease: self.damage_decrease,
            spell,
        }))
    }

    fn get_name(&self) -> String {
        "Dupe: ".to_string() + &self.bullet_count.to_string()
    }

    fn get_desc(&self) -> String {
        "The next spells gains: \n".to_string()
            + "- Bullets: +"
            + &self.bullet_count.to_string()
            + "\n"
            + "- Spread: +"
            + &self.spread_increase.to_string()
            + "\n"
            + "- Damage: "
            + &*(100. * (&self.damage_decrease - 1.)).to_string()
            + "%"
    }
}

#[derive(Debug, Clone)]
pub struct Duplicate {
    pub spread_increase: f32,
    pub bullet_count: i32,
    pub damage_decrease: f32,
    pub spell: Arc<dyn SpellEffect>,
}
impl SpellEffect for Duplicate {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let spread_increase = self.spread_increase;
        let damage_decrease = self.damage_decrease;
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            // get ProjectileDamage component
            if let Some(mut projectile_damage) = mod_world.get_mut::<ProjectileDamage>(e) {
                projectile_damage.damage *= damage_decrease;
            };
        });

        context.add_modifier("Duplicate", modifier);
        context.values.spread += spread_increase;

        info!("Cast Duplicate Mod");

        for _ in 0..self.bullet_count {
            self.spell.cast(context, world);
        }
    }
}

#[derive(Clone)]
pub struct LifetimeData {
    pub lifetime_increase: f32,
}
impl SpellData for LifetimeData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>> {
        let spell = iter.next()?.data.build(iter)?;
        Some(Arc::new(Lifetime {
            lifetime_increase: self.lifetime_increase,
            spell,
        }))
    }

    fn get_name(&self) -> String {
        "Lifetime Up".to_string()
    }

    fn get_desc(&self) -> String {
        "The next spells gains: \n".to_string()
            + "- Lifetime: +"
            + &(100. * (self.lifetime_increase - 1.)).to_string()
            + "%"
    }
}

#[derive(Debug, Clone)]
pub struct Lifetime {
    pub lifetime_increase: f32,
    pub spell: Arc<dyn SpellEffect>,
}
impl SpellEffect for Lifetime {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World) {
        let lifetime_increase = self.lifetime_increase;
        let modifier: SpellModifier = Box::new(move |e: Entity, mod_world: &mut World| {
            // get ProjectileLifetime component
            if let Some(mut projectile_lifetime) = mod_world.get_mut::<ProjectileLifetime>(e) {
                let new_lifetime =
                    projectile_lifetime.lifetime.duration().as_secs_f32() * lifetime_increase;
                projectile_lifetime
                    .lifetime
                    .set_duration(Duration::from_secs_f32(new_lifetime));
            };
        });

        context.add_modifier("Lifetime Mod", modifier);
        info!("Cast Lifetime Mod");
        self.spell.cast(context, world);
    }
}
