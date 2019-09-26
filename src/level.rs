//! A module for a whole level
//! This is mostly a wrapper around a grid and rule manager

use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

use crate::grid::*;
use crate::rules::*;
use crate::square::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    VERTICAL,
    HORIZONTAL,
}

pub const VERTICAL: Direction = Direction::VERTICAL;
pub const HORIZONTAL: Direction = Direction::HORIZONTAL;

/// A position in 2D coordinates. (0, 0) being the top left corner
pub type Position = (usize, usize);

#[derive(Clone, Debug)]
pub struct Level {
    grid: Grid<Square>,                   // usize -> Square
    rules: RuleManager,                   // Entity -> Vec<Text>
    tracking: [Vec<usize>; LAYERED_SQUARES_NUMBER], // LayeredSquare -> Vec<usize>
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            rules: RuleManager::default(),
            tracking: array_init::array_init(|_| vec![]),
        }
    }

    /// Adds a layered square to the specified square in 2D coordinates
    pub fn add_square(&mut self, square: LayeredSquare, pos: Position) {
        self.add_square_internal(square, self.grid.index(pos));
    }

    /// Adds a layered square to the specified square in 1D coordinates
    pub fn add_square_internal(&mut self, square: LayeredSquare, pos: usize) {
        // Adding to the grid
        self.grid[pos].add_layer(square);
        // Adding to the tracking
        self[square].push(pos);
    }

    pub fn add_square_line(
        &mut self,
        square: LayeredSquare,
        mut pos: Position,
        length: usize,
        dir: Direction,
    ) {
        let offset = Position::from(dir);
        for _ in 0..length {
            self.add_square(square, pos);
            pos = offset_pos(pos, offset);
        }
    }

    /// Adds a rule to the current level
    pub fn add_rule(&mut self, rule: [Text; 3], mut pos: Position, dir: Direction) {
        let offset = Position::from(dir);
        // Placing the squares
        for &text in &rule {
            self.add_square(LayeredSquare::from(text), pos);
            pos = offset_pos(pos, offset);
        }
        // Creating the rule
        // TODO correct parsing
        let entity =
            Entity::try_from(rule[0]).expect("The first element of a rule should be an entity");
        self.rules[entity].push(rule[2]);
    }
}

// Converts a direction to an 2D offset
impl From<Direction> for Position {
    fn from(dir: Direction) -> Self {
        match dir {
            VERTICAL => (0, 1),
            HORIZONTAL => (1, 0),
        }
    }
}

fn offset_pos(pos: Position, offset: Position) -> Position {
    (pos.0 + offset.0, pos.1 + offset.1)
}

impl Index<LayeredSquare> for Level {
    type Output = Vec<usize>;
    fn index(&self, square: LayeredSquare) -> &Self::Output {
        &self.tracking[usize::from(square)]
    }
}

impl IndexMut<LayeredSquare> for Level {
    fn index_mut(&mut self, square: LayeredSquare) -> &mut Self::Output {
        &mut self.tracking[usize::from(square)]
    }
}
