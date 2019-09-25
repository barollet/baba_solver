//! A rule manager, the rules are manually updated by the level interface

use std::ops::{Index, IndexMut};
use std::convert::TryFrom;

use either::*;

use crate::square::*;

pub struct RuleManager {
    rules: [Vec<Text>; Text::VARIANT_COUNT],
}

impl Default for RuleManager {
    fn default() -> Self {
        let mut manager = Self {
            rules: array_init::array_init(|_| vec!()),
        };
        manager.add_rule(Text::TEXT, Text::PUSH);

        manager
    }
}

impl Index<Text> for RuleManager {
    type Output = Vec<Text>;

    fn index(&self, text: Text) -> &Self::Output {
        &self.rules[text as usize]
    }
}

impl IndexMut<Text> for RuleManager {
    fn index_mut<'a>(&'a mut self, text: Text) -> &'a mut Self::Output {
        &mut self.rules[text as usize]
    }
}

impl RuleManager {
    pub fn add_rule(&mut self, entity: Text, property: Text) {
        if !self[entity].contains(&property) {
            self[entity].push(property);
        }
    }

    pub fn has_property(&self, square: Square, property: Text) {
        self[Text::from(Either::try_from(square))].contains(&property);
    }
}
