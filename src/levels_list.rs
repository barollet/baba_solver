//! A definition of all the levels

use crate::level::*;
use crate::square::*;

const LEVELS_NUMBER: usize = 1;

lazy_static! {
    pub static ref LEVELS_LIST: [Level; LEVELS_NUMBER] = [
        level_1(),
    ];
}

fn level_1() -> Level {
    let mut level = Level::new(15, 11);

    // Walls
    level.add_square_line(Entity::WALL.into(), (2, 3), 11, HORIZONTAL);
    level.add_square_line(Entity::WALL.into(), (2, 7), 11, HORIZONTAL);
    // Rocks
    level.add_square_line(Entity::ROCK.into(), (7, 4), 3, VERTICAL);
    // Baba and flag
    level.add_square(Entity::BABA.into(), (3, 5));
    level.add_square(Entity::FLAG.into(), (11, 5));
    // Rules
    level.add_rule([TBABA, TIS, TYOU], (2, 1), HORIZONTAL);
    level.add_rule([TFLAG, TIS, TWIN], (10, 1), HORIZONTAL);
    level.add_rule([TWALL, TIS, TSTOP], (2, 9), HORIZONTAL);
    level.add_rule([TROCK, TIS, TPUSH], (10, 9), HORIZONTAL);

    level
}
