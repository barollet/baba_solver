//! A module interpreting a move sequence in a given level

use crate::level::*;
use crate::square::*;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

pub const LEFT: Move = Move::Left;
pub const RIGHT: Move = Move::Right;
pub const UP: Move = Move::Up;
pub const DOWN: Move = Move::Down;

/// State of the game after the move, still playing, win,
/// defeat or stuck
pub enum GameState {
    Ongoing,
    Win,
    Defeat,
}

impl Level {
    /// Apply the given move to all the entities
    /// that are tagged YOU
    pub fn apply_move(&mut self, m: Move) -> GameState {
        // finding you(s)
        let you_entities: Vec<&Entity> = ENTITIES
            .iter()
            .filter(|&&e| self.rules[e][usize::from(TYOU)])
            .collect();
        for &you in you_entities {
            for elem in self.grid[LayeredSquare::from(you)].clone() {
                self.move_entity(m, (elem, you));
            }
        }
        GameState::Ongoing
    }

    /// Main physics function applying all the constraints to a moving entity
    fn move_entity(&mut self, m: Move, entity: (usize, Entity)) -> GameState {
        GameState::Ongoing
    }

    pub fn apply_move_sequence(&mut self, ms: Vec<Move>) {
        for &m in &ms {
            self.apply_move(m);
        }
    }
}
