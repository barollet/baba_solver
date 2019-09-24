//! A helper module for the grid squares and a list of entities and text

use either::*;
use enum_primitive::FromPrimitive;
use std::convert::TryFrom;

const FIRST_ENTITY_INDEX: u32 = 10;
bitflags! {
    #[derive(Default)]
    pub struct Square: u32 {
        const BABA = 1;
        const IS = 1 << 2;
        const YOU = 1 << 3;
        const FLAG = 1 << 4;
        const WIN = 1 << 5;
        const WALL = 1 << 6;
        const STOP = 1 << 7;
        const ROCK = 1 << 8;
        const PUSH = 1 << 9;

        const BABA_ENT = 1 << 10;
        const ROCK_ENT = 1 << 11;
        const FLAG_ENT = 1 << 12;
        const WALL_ENT = 1 << 13;
    }
}

enum_from_primitive! {
    #[repr(u8)]
    pub enum Text {
        BABA = 0,
        IS,
        YOU,
        FLAG,
        WIN,
        WALL,
        STOP,
        ROCK,
        PUSH,
    }
}

enum_from_primitive! {
    #[repr(u8)]
    pub enum Entity {
        BABA = 0,
        ROCK,
        FLAG,
        WALL,
    }
}

pub type SquareConversionError = ();

impl TryFrom<Square> for Text {
    type Error = SquareConversionError;
    fn try_from(square: Square) -> Result<Self, Self::Error> {
        let index = square.bits().trailing_zeros();
        Text::from_u32(index).ok_or(())
    }
}

impl TryFrom<Square> for Entity {
    type Error = SquareConversionError;
    fn try_from(square: Square) -> Result<Self, Self::Error> {
        let index = square.bits().trailing_zeros();
        Entity::from_u32(index - FIRST_ENTITY_INDEX).ok_or(())
    }
}

impl TryFrom<Square> for Either<Text, Entity> {
    type Error = SquareConversionError;
    fn try_from(square: Square) -> Result<Self, Self::Error> {
        match Text::try_from(square) {
            Ok(text) => Ok(Either::Left(text)),
            Err(()) => Entity::try_from(square).map(|entity| Either::Right(entity)),
        }
    }
}

impl IntoIterator for Square {
    type Item = Square;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator {
            value: self.bits(),
        }
    }
}

pub struct SquareIterator {
    value: u32,
}

impl Iterator for SquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value != 0 {
            let unitary_square = self.value & self.value.overflowing_neg().0;
            Some(Square::from_bits_truncate(unitary_square))
        } else {
            None
        }
    }
}
