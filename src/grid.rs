//! A module for a generic 2D grid
//! This is typicaly used to store the level grid and entity positions

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
