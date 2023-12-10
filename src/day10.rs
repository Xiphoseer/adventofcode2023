use std::{collections::BTreeMap, path::Path};

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

#[derive(Debug, Copy, Clone)]
pub enum Edge {
    // `F`
    SouthEast,
    // `L`
    NorthEast,
    // `7`
    SouthWest,
    // `J`
    NorthWest,
    // `|`
    NorthSouth,
    // `-`
    EastWest,
}

pub struct Map {
    bytes: Vec<u8>,
    stride: usize,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Map {
    pub fn new(path: &Path) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let width = bytes.iter().position(|x| *x == b'\n').expect("newline");
        let stride = width + 1;
        let height = bytes.len() / stride;
        let start = bytes.iter().position(|x| *x == b'S').expect("start");
        Self {
            bytes,
            width,
            height,
            start: (start % stride, start / stride),
            stride,
        }
    }

    pub fn turn(&self, dir: Dir, (x, y): (usize, usize)) -> Option<(Dir, Edge)> {
        let p = self.bytes[y * self.stride + x];
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
    let (len, border) = scan(&map).unwrap();
    let in_fields = area(&map, &border);
    (len, in_fields)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Out,
    BottomBorder,
    In,
    TopBorder,
}

fn area(map: &Map, border: &BTreeMap<(usize, usize), Edge>) -> usize {
    let mut in_fields = 0;
    eprintln!("Color: {}x{}", map.width, map.height);
    for y in 0..map.height {
        use Edge::*;
        use State::*;
        let mut state = Out;
        for x in 0..map.width {
            if let Some(edge) = border.get(&(x, y)) {
                eprintln!("{state:?} {edge:?} {x},{y}");
                state = match (state, *edge) {
                    (Out, SouthEast) => TopBorder,
                    (Out, NorthEast) => BottomBorder,
                    (Out, SouthWest | NorthWest | EastWest) => unreachable!(),
                    (Out, NorthSouth) => In,
                    (TopBorder, SouthEast | NorthEast | NorthSouth) => unreachable!(),
                    (TopBorder, SouthWest) => Out,
                    (TopBorder, NorthWest) => In,
                    (TopBorder, EastWest) => TopBorder,
                    (In, SouthEast) => BottomBorder,
                    (In, NorthEast) => TopBorder,
                    (In, SouthWest | NorthWest | EastWest) => unreachable!(),
                    (In, NorthSouth) => Out,
                    (BottomBorder, SouthEast | NorthEast | NorthSouth) => unreachable!(),
                    (BottomBorder, SouthWest) => In,
                    (BottomBorder, NorthWest) => Out,
                    (BottomBorder, EastWest) => BottomBorder,
                };
            } else if state == In {
                in_fields += 1;
            }
        }
    }
    in_fields
}

type EdgeMap = BTreeMap<(usize, usize), Edge>;
fn scan(map: &Map) -> Option<(usize, EdgeMap)> {
    let (sx, sy) = map.start;
    let mut border = BTreeMap::new();
    eprintln!("{sx},{sy}");
    use Dir::*;
    for start_dir in [Up, Down, Left, Right] {
        let mut dir = start_dir;
        eprintln!("{dir:?}");
        let (mut x, mut y) = (sx, sy);
        let mut count = 0;
        if let Some(len) = loop {
            (x, y) = dir.go((x, y));
            eprintln!("{dir:?} to {x},{y}");
            count += 1;
            if (x, y) == (sx, sy) {
                let start_edge = start_edge(start_dir, dir);
                border.insert((sx, sy), start_edge);
                break Some(count);
            }
            if let Some((next_dir, edge)) = map.turn(dir, (x, y)) {
                border.insert((x, y), edge);
                dir = next_dir;
            } else {
                break None;
            }
        } {
            return Some((len / 2, border));
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
