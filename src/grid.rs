//! A module for a generic 2D grid
//! This is typicaly used to store the level grid and entity positions

use std::ops::{Index, IndexMut};
use std::fmt;

use crate::interpreter::Move;
use crate::level::*;

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    elems: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new_with_default(width: usize, height: usize, elem: T) -> Self {
        let mut elems = Vec::with_capacity(width * height);
        for _ in 0..width*height {
            elems.push(elem);
        }
        Grid {
            width,
            height,
            elems,
        }
    }
}

impl<T: Default + Copy> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid::new_with_default(width, height, T::default())
    }
}

impl<T> Grid<T> {
    pub fn left(&self, elem: usize) -> Option<usize> {
        if elem % self.width > 0 {
            Some(elem - 1)
        } else {
            None
        }
    }
    pub fn right(&self, elem: usize) -> Option<usize> {
        if elem % self.width < self.width - 1 {
            Some(elem + 1)
        } else {
            None
        }
    }
    pub fn up(&self, elem: usize) -> Option<usize> {
        if elem / self.width > 0 {
            Some(elem - self.width)
        } else {
            None
        }
    }
    pub fn down(&self, elem: usize) -> Option<usize> {
        if elem / self.width < self.height - 1 {
            Some(elem + self.width)
        } else {
            None
        }
    }
    /// Apply a move to the given element, if the move is invalid, returns None
    pub fn apply_move(&self, elem: usize, m: Move) -> Option<usize> {
        match m {
            Move::Left => self.left(elem),
            Move::Right => self.right(elem),
            Move::Up => self.up(elem),
            Move::Down => self.down(elem),
        }
    }

    /// Converts a 2D Position into an index
    pub fn index(&self, pos: Position) -> usize {
        pos.0 + pos.1 * self.width
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, elem: usize) -> &Self::Output {
        &self.elems[elem]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, elem: usize) -> &mut Self::Output {
        &mut self.elems[elem]
    }
}

// Index by 2D coordinates, width first
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, coord: (usize, usize)) -> &Self::Output {
        &self.elems[coord.0 + coord.1 * self.width]
    }
}

impl<G, T: Copy + IntoIterator<Item = G>> fmt::Debug for Grid<T> where G: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(w:{}, h:{})", self.width, self.height)?;
        for i in 0..self.height {
            // Printing a line of Square
            let line: Vec<String> = (0..self.width).map(|j| {
                // Printing all the layers in a line
                let elem: Vec<String> = self[(j, i)].into_iter().map(|l| format!("{:?}", l)).collect();
                format!("{}", elem.join(","))
            }).collect();
            writeln!(f, "{}", line.join("|"))?;
        }
        Ok(())
    }
}
