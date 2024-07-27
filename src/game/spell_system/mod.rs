use std::fmt;
use std::fmt::Debug;
use std::slice::Iter;
use std::sync::Arc;

use bevy::app::App;
use bevy::prelude::*;

use crate::game::spell_system::casting::SpellCastContext;
use crate::game::spell_system::SpellModifierNode::Node;

pub mod casting;
pub mod examples;
pub mod helpers;
pub mod spells;
pub mod storage;
pub mod triggers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        casting::plugin,
        triggers::plugin,
        storage::plugin,
        spells::plugin,
    ));
}

#[derive(Clone, Component)]
pub struct SpellComponent {
    pub data: Box<dyn SpellData>,
    pub icon_id: usize,
    // pub tier: u32, //todo
}

pub trait SpellData: Send + Sync + CloneBoxSpellData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>>;
    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;

    // fn get_desc(&self) -> String; //todo, build description in the data so it can use the numbers
    // fn can_upgrade(&self) -> bool; //todo
    // fn upgrade(&self); //todo
    // fn get_upgrade_desc(&self) -> String; //todo
}

pub trait CloneBoxSpellData {
    fn clone_box(&self) -> Box<dyn SpellData>;
}
impl<T> CloneBoxSpellData for T
where
    T: 'static + SpellData + Clone,
{
    fn clone_box(&self) -> Box<dyn SpellData> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn SpellData> {
    fn clone(&self) -> Box<dyn SpellData> {
        self.clone_box()
    }
}

pub trait SpellEffect: Send + Sync + Debug {
    fn cast(&self, context: &mut SpellCastContext, world: &mut World);
}

pub type SpellModifier = Box<dyn Fn(Entity, &mut World) + Send + Sync + 'static>;

pub enum SpellModifierNode {
    Node {
        id: String,
        modifier: SpellModifier,
        prev: Option<Arc<SpellModifierNode>>,
    },
    Root,
}
impl Debug for SpellModifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpellModifierNode::Root => {
                write!(f, "SpellMod:ROOT")
            }
            SpellModifierNode::Node {
                id,
                modifier: _,
                prev,
            } => {
                write!(f, "SpellMod:{}", id)?;

                if let Some(prev) = prev {
                    write!(f, "->")?;
                    std::fmt::Debug::fmt(&prev, f)?;
                }

                Ok(())
            }
        }
    }
}
impl SpellModifierNode {
    fn with_new(
        id: &str,
        modifier: Arc<SpellModifierNode>,
        new_modifier: SpellModifier,
    ) -> Arc<Self> {
        Arc::new(Node {
            id: String::from(id),
            modifier: new_modifier,
            prev: Some(modifier),
        })
    }

    fn apply(&self, entity: Entity, world: &mut World) {
        match self {
            SpellModifierNode::Root => {}
            SpellModifierNode::Node {
                id: _,
                modifier,
                prev,
            } => {
                modifier(entity, world);
                if let Some(ref prev) = prev {
                    prev.apply(entity, world);
                }
            }
        }
    }
}
