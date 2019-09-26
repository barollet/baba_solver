//! A helper module for the grid squares and a list of entities and text

use std::convert::TryFrom;
use enum_primitive::FromPrimitive;
use variant_count::VariantCount;

enum_from_primitive! {
    /// An entity in the world, empty and text are entities but should
    /// not be assigned manually in the world
    #[derive(PartialEq, Eq, Hash, VariantCount, Clone, Copy, Debug)]
    #[repr(u8)]
    pub enum Entity {
        BABA = 0,
        FLAG,
        WALL,
        ROCK,

        // Special values that are never assigned
        EMPTY,
        TEXT,
    }
}
enum_from_primitive! {
    /// A property that an entity can have
    #[derive(PartialEq, Eq, Hash, VariantCount, Clone, Copy, Debug)]
    #[repr(u8)]
    pub enum Property {
        YOU = 0,
        WIN,
        STOP,
        PUSH,
    }
}

/// A text is either designing an entity, a property or a keyword
#[derive(PartialEq, Eq, Hash, VariantCount, Clone, Copy, Debug)]
pub enum Text {
    Entity(Entity),
    Property(Property),
    Is,
}

/// All the individual value a square can take
#[derive(Clone, Copy, Debug)]
pub enum LayeredSquare {
    Entity(Entity),
    Text(Text),
}

/// A square is a boolean table of all the possible layered squares (superposition)
#[derive(Default, Clone, Copy)]
pub struct Square {
    value: u32,
}

impl Square {
    /// Adds a layer to the given square in place
    pub fn add_layer(&mut self, layer: LayeredSquare) {
        self.value |= usize::from(layer) as u32;
    }
}

// Text shortcut constants
pub const TBABA: Text = Text::Entity(Entity::BABA);
pub const TIS: Text = Text::Is;
pub const TYOU: Text = Text::Property(Property::YOU);
pub const TFLAG: Text = Text::Entity(Entity::FLAG);
pub const TWIN: Text = Text::Property(Property::WIN);
pub const TWALL: Text = Text::Entity(Entity::WALL);
pub const TSTOP: Text = Text::Property(Property::STOP);
pub const TROCK: Text = Text::Entity(Entity::ROCK);
pub const TPUSH: Text = Text::Property(Property::PUSH);

// Conversions between layered squares and a unique shifting index
impl From<usize> for LayeredSquare {
    fn from(index: usize) -> Self {
        // We try to convert it to an entity
        Entity::from_usize(index).map_or_else(
            // If it fails, then this is a text
            || {
                // We shift the index into "text" coordinates
                let index = index - Entity::VARIANT_COUNT;
                // We try again to convert it to an entity
                Entity::from_usize(index).map_or_else(
                    // If it fails we have a property or a keyword
                    || {
                        // We shift the index into "text property" coordinates
                        let index = index - Entity::VARIANT_COUNT;
                        // We try to convert the index into a property
                        Property::from_usize(index).map_or_else(
                            // If it fails we have a keyword
                            || LayeredSquare::Text(Text::Is),
                            // If it succeeds we have a text property
                            LayeredSquare::from,
                        )
                    },
                    // If it succeeds then we have a text refering to an entity
                    LayeredSquare::from,
                )
            },
            // If it succeeds then we return this entity
            LayeredSquare::Entity,
        )
    }
}

impl From<LayeredSquare> for usize {
    fn from(square: LayeredSquare) -> Self {
        match square {
            LayeredSquare::Entity(e) => e as usize,
            LayeredSquare::Text(t) => {
                Entity::VARIANT_COUNT + usize::from(t)
            }
        }
    }
}

impl From<Text> for usize {
    fn from(text: Text) -> Self {
        match text {
            Text::Entity(e) => e as usize,
            Text::Property(p) => Entity::VARIANT_COUNT + p as usize,
            Text::Is => Entity::VARIANT_COUNT + Property::VARIANT_COUNT + 1,
        }
    }
}

// Converts an layered square to the given entity
impl From<LayeredSquare> for Entity {
    fn from(square: LayeredSquare) -> Self {
        match square {
            LayeredSquare::Entity(e) => e,
            LayeredSquare::Text(_) => Entity::TEXT,
        }
    }
}


impl TryFrom<Text> for Entity {
    type Error = ();
    fn try_from(t: Text) -> Result<Self, Self::Error> {
        match t {
            Text::Entity(e) => Ok(e),
            _ => Err(()),
        }
    }
}

impl From<Property> for Text {
    fn from(p: Property) -> Self {
        Self::Property(p)
    }
}

impl From<Entity> for Text {
    fn from(e: Entity) -> Self {
        Self::Entity(e)
    }
}

impl From<Text> for LayeredSquare {
    fn from(t: Text) -> Self {
        Self::Text(t)
    }
}

impl From<Property> for LayeredSquare {
    fn from(p: Property) -> Self {
        Self::from(Text::from(p))
    }
}

impl From<Entity> for LayeredSquare {
    fn from(e: Entity) -> Self {
        Self::Entity(e)
    }
}

impl IntoIterator for Square {
    type Item = LayeredSquare;
    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator { value: self.value }
    }
}

pub struct SquareIterator {
    value: u32,
}

impl Iterator for SquareIterator {
    type Item = LayeredSquare;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value != 0 {
            let unitary_square = self.value & self.value.overflowing_neg().0;
            Some(LayeredSquare::from(unitary_square as usize))
        } else {
            None
        }
    }
}
