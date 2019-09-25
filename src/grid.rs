//! A module for a generic 2D grid
//! This is typicaly used to store the level grid and entity positions

use std::ops::{Index, IndexMut};

use crate::interpreter::Move;

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
    pub fn move(&self, elem: usize, m: Move) -> Option<usize> {
        match m {
            Move::Left => self.left(elem),
            Move::Right => self.right(elem),
            Move::Up => self.up(elem),
            Move::Down => self.down(elem),
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, elem: usize) -> &Self::Output {
        &self.elems[elem]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut<'a>(&'a mut self, elem: usize) -> &'a mut Self::Output {
        &mut self.elems[elem]
    }
}
