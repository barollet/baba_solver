//! A module for a whole level
//! This is mostly a wrapper around a grid and rule manager

use crate::grid::*;
use crate::rules::*;
use crate::square::*;


pub struct Level {
    grid: Grid<Square>,
    rules: RuleManager,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            rules: RuleManager::default(),
        }
    }
}
