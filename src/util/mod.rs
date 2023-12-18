use std::{
    ops::{Deref, Range},
    path::Path,
};

use num::Integer;

pub mod area;

pub type Pos<A> = (A, A);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_usize(i: usize) -> Self {
        [Self::Right, Self::Up, Self::Left, Self::Down][i]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapDimensions {
    size: (usize, usize),
    stride: usize,
}

impl MapDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            stride: width + 1,
        }
    }

    pub fn of(map: &str) -> Self {
        assert_eq!(map.chars().last(), Some('\n'));
        let width = map.split_once('\n').unwrap().0.len();
        let stride = width + 1;
        let height = map.len() / stride;
        Self {
            size: (width, height),
            stride,
        }
    }

    pub fn index(&self, (x, y): (usize, usize)) -> usize {
        y * self.stride() + x
    }

    pub fn of_index(&self, i: usize) -> (usize, usize) {
        let (y, x) = i.div_rem(&self.stride); // x = i % stride, y = i / stride
        (x, y)
    }

    pub fn xrange(&self) -> Range<usize> {
        0..self.width()
    }

    pub fn yrange(&self) -> Range<usize> {
        0..self.height()
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn turn_left(
        &self,
        from: Direction,
        (x, y): (usize, usize),
    ) -> Option<(Direction, (usize, usize))> {
        match from {
            Direction::Right => self.north_of((x, y)).map(|p| (Direction::Up, p)),
            Direction::Up => self.west_of((x, y)).map(|p| (Direction::Left, p)),
            Direction::Left => self.south_of((x, y)).map(|p| (Direction::Down, p)),
            Direction::Down => self.east_of((x, y)).map(|p| (Direction::Right, p)),
        }
    }

    pub fn turn_right(
        &self,
        from: Direction,
        (x, y): (usize, usize),
    ) -> Option<(Direction, (usize, usize))> {
        match from {
            Direction::Right => self.south_of((x, y)).map(|p| (Direction::Down, p)),
            Direction::Up => self.east_of((x, y)).map(|p| (Direction::Right, p)),
            Direction::Left => self.north_of((x, y)).map(|p| (Direction::Up, p)),
            Direction::Down => self.west_of((x, y)).map(|p| (Direction::Left, p)),
        }
    }

    pub fn go_straight(&self, from: Direction, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match from {
            Direction::Right => self.east_of((x, y)),
            Direction::Up => self.north_of((x, y)),
            Direction::Left => self.west_of((x, y)),
            Direction::Down => self.south_of((x, y)),
        }
    }

    pub fn north_of(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if y > 0 {
            Some((x, y - 1))
        } else {
            None
        }
    }

    pub fn south_of(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if y + 1 < self.height() {
            Some((x, y + 1))
        } else {
            None
        }
    }

    pub fn west_of(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if x > 0 {
            Some((x - 1, y))
        } else {
            None
        }
    }

    pub fn east_of(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if x + 1 < self.width() {
            Some((x + 1, y))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    inner: String,
    dim: MapDimensions,
}

impl Map {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::of(text))
    }

    pub fn of(inner: String) -> Self {
        Self {
            dim: MapDimensions::of(&inner),
            inner,
        }
    }

    pub fn dim(&self) -> &MapDimensions {
        &self.dim
    }

    pub(crate) fn ascii_num_at(&self, (x, y): (usize, usize)) -> u8 {
        self.as_bytes()[self.dim.index((x, y))] - b'0'
    }
}

impl Deref for Map {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
