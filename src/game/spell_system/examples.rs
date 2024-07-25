/////////////////////////////
// EXAMPLE IMPLEMENTATIONS //
/////////////////////////////

// pub struct CastTriggeredSpells {
//     trigger: Trigger<SpellTriggerEvent>,
//
//
// }

// pub struct ExampleModifierEffect {
//     pub modified_spell: Arc<dyn SpellEffect>,
// }
// impl SpellEffect for ExampleModifierEffect {
//     fn get_name(&self) -> &str {
//         todo!()
//     }
//
//     fn cast(&self, caster: Entity, world: &mut World) -> SpellSet {
//         let spell_set = self.modified_spell.cast(caster, world);
//
//         //todo: use world scope to get/add components on the spell_system in the spellset
//         //e.g. get damage components and add +5
//
//         return spell_set;
//     }
// }
//
// pub struct MulticastSpellsEffect {
//     pub spell_system: Vec<Arc<dyn SpellEffect>>,
// }
// impl SpellEffect for MulticastSpellsEffect {
//     fn get_name(&self) -> &str {
//         todo!()
//     }
//
//     fn cast(&self, caster: Entity, world: &mut World) -> SpellSet {
//         let mut spell_set : Vec<SpellSet> = Vec::new();
//         for spell in &self.spell_system {
//             spell_set.push(spell.cast(caster, world));
//         }
//         return SpellSet::Set(spell_set);
//     }
// }
