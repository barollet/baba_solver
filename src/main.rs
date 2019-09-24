#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;

mod grid;
mod level;
mod rules;
mod interpreter;
mod square;

use level::*;

fn main() {
    println!("Hello, world!");
    let level = Level::new(5, 5);
}
