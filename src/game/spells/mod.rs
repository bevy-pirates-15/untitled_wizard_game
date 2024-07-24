use std::fmt;
use std::fmt::Debug;
use std::slice::Iter;
use std::sync::Arc;

use bevy::app::App;
use bevy::prelude::{Entity, World};

use crate::game::spells::casting::SpellCastContext;
use crate::game::spells::SpellModifierNode::Node;

pub mod casting;
pub mod examples;
pub mod helpers;
pub mod triggers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((casting::plugin, triggers::plugin));
}

#[derive(Clone)]
pub struct SpellComponent {
    data: Arc<dyn SpellData>,
    // pub icon: String, //todo
    // pub tier: u32, //todo
}

pub trait SpellData {
    fn build(&self, iter: &mut Iter<SpellComponent>) -> Option<Arc<dyn SpellEffect>>;
    // fn get_desc(&self) -> String; //todo, build description in the data so it can use the numbers

    // fn can_upgrade(&self) -> bool; //todo
    // fn upgrade(&self); //todo
    // fn get_upgrade_desc(&self) -> String; //todo
}

pub trait SpellEffect: Send + Sync + Debug {
    fn get_name(&self) -> &str;
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
