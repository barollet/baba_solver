//! A rule manager, the rules are manually updated by the level interface
//! This a 2D boolean table indexed by entity and text

use std::ops::{Index, IndexMut};

use crate::square::*;

type TextLine = [bool; TEXTS_NUMBER];

#[derive(Clone, Debug)]
pub struct RuleManager([TextLine; Entity::VARIANT_COUNT]);

impl Default for RuleManager {
    fn default() -> Self {
        let mut manager = Self(array_init::array_init(|_| [false; TEXTS_NUMBER]));
        manager.add_rule(Entity::TEXT, TPUSH);

        manager
    }
}

impl Index<Entity> for RuleManager {
    type Output = TextLine;

    fn index(&self, entity: Entity) -> &Self::Output {
        &self.0[entity as usize]
    }
}

impl IndexMut<Entity> for RuleManager {
    fn index_mut(&mut self, entity: Entity) -> &mut Self::Output {
        &mut self.0[entity as usize]
    }
}

impl RuleManager {
    /// Sets the given rule as true
    pub fn add_rule(&mut self, entity: Entity, property: Text) {
        self[entity][usize::from(property)] = true;
    }

    /// Returns if the given square has the given property
    pub fn square_has_property(&self, square: Square, property: Text) -> bool {
        square
            .into_iter()
            .any(|layer| self[Entity::from(layer)][usize::from(Text::from(property))])
    }
}
