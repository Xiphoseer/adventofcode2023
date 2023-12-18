use std::{collections::BTreeMap, path::Path};

use crate::util::{
    area::{area, Edge, EdgeMap, Noop},
    MapDimensions,
};

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn go(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        }
    }
}

pub struct Map {
    bytes: Vec<u8>,
    dim: MapDimensions,
    start: (usize, usize),
}

impl Map {
    pub fn new(path: &Path) -> Self {
        let text = std::fs::read_to_string(path).unwrap();
        let dim = MapDimensions::of(&text);
        let bytes = text.into_bytes();
        let start = bytes.iter().position(|x| *x == b'S').expect("start");
        Self {
            bytes,
            dim,
            start: dim.of_index(start),
        }
    }

    pub fn turn(&self, dir: Dir, (x, y): (usize, usize)) -> Option<(Dir, Edge)> {
        let p = self.bytes[self.dim.index((x, y))];
        use Dir::*;
        match (dir, p) {
            (Up, b'F') => Some((Right, Edge::SouthEast)),
            (Up, b'7') => Some((Left, Edge::SouthWest)),

            (Down, b'J') => Some((Left, Edge::NorthWest)),
            (Down, b'L') => Some((Right, Edge::NorthEast)),

            (Right, b'7') => Some((Down, Edge::SouthWest)),
            (Right, b'J') => Some((Up, Edge::NorthWest)),

            (Left, b'F') => Some((Down, Edge::SouthEast)),
            (Left, b'L') => Some((Up, Edge::NorthEast)),

            (Up | Down, b'|') => Some((dir, Edge::NorthSouth)),
            (Left | Right, b'-') => Some((dir, Edge::EastWest)),

            (_, b'S') => panic!("START"),
            _ => None,
        }
    }
}

pub fn run(path: &Path) -> (usize, usize) {
    let map = Map::new(path);
    let border = scan(&map).unwrap();
    let len = border.len() / 2;
    let in_fields = area(0..map.dim.width(), 0..map.dim.height(), &border, &mut Noop);
    (len, in_fields)
}

fn scan(map: &Map) -> Option<EdgeMap<usize>> {
    let (sx, sy) = map.start;
    let mut border = BTreeMap::new();
    eprintln!("{sx},{sy}");
    use Dir::*;
    for start_dir in [Up, Down, Left, Right] {
        let mut dir = start_dir;
        eprintln!("{dir:?}");
        let (mut x, mut y) = (sx, sy);
        loop {
            (x, y) = dir.go((x, y));
            eprintln!("{dir:?} to {x},{y}");
            if (x, y) == (sx, sy) {
                let start_edge = start_edge(start_dir, dir);
                border.insert((sx, sy), start_edge);
                return Some(border);
            }
            if let Some((next_dir, edge)) = map.turn(dir, (x, y)) {
                border.insert((x, y), edge);
                dir = next_dir;
            } else {
                break;
            }
        }
    }
    None
}

fn start_edge(start_dir: Dir, dir: Dir) -> Edge {
    use Dir::*;
    match (start_dir, dir) {
        (Up, Up) | (Down, Down) => Edge::NorthSouth,
        (Up, Down) | (Down, Up) => unreachable!(),
        (Up, Left) | (Left, Up) => Edge::NorthEast,
        (Up, Right) | (Right, Up) => Edge::NorthWest,
        (Down, Left) | (Left, Down) => Edge::SouthEast,
        (Down, Right) | (Right, Down) => Edge::SouthWest,
        (Left, Left) | (Right, Right) => Edge::EastWest,
        (Left, Right) | (Right, Left) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn example() {
        let (len, in_fields) = run(Path::new("res/day10/example.txt"));
        assert_eq!(len, 8);
        assert_eq!(in_fields, 1);
    }

    #[test]
    fn input() {
        let (len, in_fields) = run(Path::new("res/day10/input.txt"));
        assert_eq!(len, 6951);
        assert_eq!(in_fields, 563);
    }
}
