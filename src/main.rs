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

fn main() {
    println!("Hello, world!");
    let level = &LEVELS_LIST[0];
}
