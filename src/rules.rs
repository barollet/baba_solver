//! A rule manager, the rules are manually updated by the level interface

use crate::square::*;
use std::collections::HashMap;


// Conversion between unitary squares and the according subtype

pub struct RuleManager {
    rules: HashMap<Square, Square>,
}

impl Default for RuleManager {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
}
