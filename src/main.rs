#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;

mod grid;
mod interpreter;
mod level;
mod levels_list;
mod rules;
mod square;

use interpreter::*;
use levels_list::*;

fn main() {
    println!("Hello, world!");
    let level = &mut LEVELS_LIST[0].clone();
    let result = level.apply_move_sequence(vec![
        LEFT, LEFT, UP, UP, UP, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, RIGHT,
        RIGHT, RIGHT, RIGHT, DOWN, DOWN, DOWN, LEFT, LEFT,
    ]);

    dbg!(result);
    //dbg!(level);

    let level = &mut LEVELS_LIST[0].clone();
    let result = level.apply_move_sequence(vec![
        RIGHT, RIGHT, RIGHT, RIGHT, RIGHT, DOWN, RIGHT, RIGHT, RIGHT, UP,
    ]);

    dbg!(result);
    dbg!(level);
}
