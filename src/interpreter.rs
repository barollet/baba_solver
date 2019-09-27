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
/// defeat or stuck. None means ongoing
pub type GameState = Option<EndState>;
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EndState {
    Win,
    Defeat,
}
const ONGOING: GameState = None;
const WIN: GameState = Some(EndState::Win);
const DEFEAT: GameState = Some(EndState::Defeat);

fn update_game_state(result: GameState, game_state: GameState) -> GameState {
    if let ONGOING = result {
        game_state
    } else if let WIN = result {
        WIN
    } else if let WIN = game_state {
        WIN
    } else {
        DEFEAT
    }
}

/// Checks if a winning or loosing condition is triggerd on a given entity
fn check_end(old_level: &Level, pos: usize, entity: Entity) -> GameState {
    // If the entity is marked as you
    if old_level.rules[entity][usize::from(TYOU)] {
        // Win has a higher priority
        if old_level
            .rules
            .square_has_property(old_level.grid[pos], TWIN)
        {
            WIN
        } else {
            ONGOING
        }
    } else {
        ONGOING
    }
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
        let old_level = self.clone();

        // Folding the game state result for each entity type that are tagged you
        you_entities.iter().fold(ONGOING, |game_state, &&you| {
            // For each world entity we apply the move
            let you_result = old_level.grid[LayeredSquare::from(you)].iter().fold(
                ONGOING,
                |game_state, &elem| {
                    update_game_state(self.move_entity(m, (elem, you), &old_level).0, game_state)
                },
            );
            update_game_state(you_result, game_state)
        })
    }

    /// Main physics function applying all the constraints to a moving entity
    /// Returns if the game ends and wether the entity could move
    fn move_entity(
        &mut self,
        m: Move,
        entity: (usize, Entity),
        old_level: &Level,
    ) -> (GameState, bool) {
        let (pos, entity) = entity;
        // We check that the move ends inside the grid boundary
        if let Some(dest) = self.grid.apply_move(pos, m) {
            // If there is a layer tagged STOP we cannot go
            if old_level
                .rules
                .square_has_property(old_level.grid[pos], TSTOP)
            {
                return (ONGOING, false);
            }

            // If there is a layer tagged PUSH we try to recursively push on all the line
            for layer in old_level.grid[dest]
                .into_iter()
                .filter(|&layer| old_level.rules[Entity::from(layer)][usize::from(TPUSH)])
            {
                let result = self.move_entity(m, (dest, Entity::from(layer)), old_level);
                // If the push succeeds we can go there and update the result
                if result.1 {
                    self.move_internal(LayeredSquare::from(entity), pos, dest);
                    return (
                        update_game_state(check_end(old_level, dest, entity), result.0),
                        true,
                    );
                }
            }

            // TODO other rules

            // If there is nothing special we just move
            self.move_internal(LayeredSquare::from(entity), pos, dest);
            (check_end(old_level, dest, entity), true)
        } else {
            (ONGOING, false)
        }
    }

    pub fn apply_move_sequence(&mut self, ms: Vec<Move>) -> GameState {
        ms.iter().fold(ONGOING, |game_state, &m| {
            update_game_state(self.apply_move(m), game_state)
        })
    }

    /// Internaly applies a move without checks, updating the according layers and rules
    fn move_internal(&mut self, layer: LayeredSquare, start: usize, dest: usize) {
        self.remove_layer_internal(layer, start);
        self.add_layer_internal(layer, dest);
    }
}
