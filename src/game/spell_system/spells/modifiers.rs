use std::slice::Iter;
use std::sync::Arc;

use bevy::log::info;
use bevy::prelude::{Entity, World};

use crate::game::projectiles::ProjectileDamage;
use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::{SpellComponent, SpellData, SpellEffect, SpellModifier};

pub(super) fn get_spells() -> Vec<SpellComponent> {
    vec![SpellComponent {
        data: Box::new(DmgUpSpellModData {
            damage_increase: 0.5,
        }),
        icon_id: 12,
    }]
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
        "DMGUP Mod".to_string()
    }

    fn get_desc(&self) -> String {
        "Increases the damage of the spell it contains by: ".to_string()
            + &self.damage_increase.to_string()
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
