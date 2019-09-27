#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;

mod grid;
mod level;
mod rules;
mod interpreter;
mod square;
mod levels_list;

use levels_list::*;
use interpreter::*;

fn main() {
    println!("Hello, world!");
    let level = &LEVELS_LIST[0];
    level.clone().apply_move_sequence(vec![
        LEFT,
        LEFT,
        UP,
        UP,
        UP,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        RIGHT,
        DOWN,
        DOWN,
        DOWN,
        LEFT,
        LEFT,
        ]);
}
