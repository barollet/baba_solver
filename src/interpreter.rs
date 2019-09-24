//! A module interpreting a move sequence in a given level

use crate::level::*;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Level {
    pub fn apply_move(&mut self, m: Move) {

    }

    pub fn apply_move_sequence(&mut self, ms: Vec<Move>) {
        for &m in &ms {
            self.apply_move(m);
        }
    }
}
