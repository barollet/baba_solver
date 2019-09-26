//! A rule manager, the rules are manually updated by the level interface

use std::ops::{Index, IndexMut};

use crate::square::*;

#[derive(Clone, Debug)]
pub struct RuleManager {
    rules: [Vec<Text>; Entity::VARIANT_COUNT],
}

impl Default for RuleManager {
    fn default() -> Self {
        let mut manager = Self {
            rules: array_init::array_init(|_| vec!()),
        };
        manager.add_rule(Entity::TEXT, TPUSH);

        manager
    }
}

impl Index<Entity> for RuleManager {
    type Output = Vec<Text>;

    fn index(&self, text: Entity) -> &Self::Output {
        &self.rules[text as usize]
    }
}

impl IndexMut<Entity> for RuleManager {
    fn index_mut(&mut self, text: Entity) -> &mut Self::Output {
        &mut self.rules[text as usize]
    }
}

impl RuleManager {
    pub fn add_rule(&mut self, entity: Entity, property: Text) {
        if !self[entity].contains(&property) {
            self[entity].push(property);
        }
    }

    // Returns if the given individual square has the given property
    pub fn has_property(&self, square: LayeredSquare, property: Property) {
        self[Entity::from(square)].contains(&Text::from(property));
    }
}
