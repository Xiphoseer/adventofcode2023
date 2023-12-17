use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
    path::Path,
};

use crate::util::{Direction, Map, MapDimensions};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Node {
    pos: (usize, usize),
    dir: Direction,
    count: usize,
}

impl Node {
    pub fn pack(&self, dim: &MapDimensions) -> usize {
        (dim.index((self.pos.0, self.pos.1)) << 10)
            | (self.dir.as_usize() << 8)
            | (self.count & 0xFF)
    }

    pub fn unpack(dim: &MapDimensions, node: usize) -> Self {
        Node {
            pos: dim.of_index(node >> 10),
            dir: Direction::from_usize((node >> 8) & 0b11),
            count: node & 0xFF,
        }
    }

    fn new((x, y): (usize, usize), dir: Direction, count: usize) -> Self {
        Self {
            pos: (x, y),
            dir,
            count,
        }
    }
}

type UpdateFn = fn(
    &Map,
    Direction,
    (usize, usize),
    usize,
    usize,
    &mut BTreeSet<usize>,
    &mut BinaryHeap<(Reverse<usize>, usize)>,
);

pub fn run(path: &Path, is_part2: bool) -> usize {
    let update: UpdateFn = match is_part2 {
        false => update_part1,
        true => update_part2,
    };
    let map = Map::open(path).unwrap();
    let dim = map.dim();
    let w1 = map.ascii_num_at((1, 0));
    let w2 = map.ascii_num_at((0, 1));
    let mut seen = BTreeSet::new();
    use Direction::*;
    let mut todo = BinaryHeap::<(Reverse<usize>, usize)>::from([
        (Reverse(w1 as usize), Node::new((1, 0), Right, 1).pack(dim)),
        (Reverse(w2 as usize), Node::new((0, 1), Down, 1).pack(dim)),
    ]);
    let target = (dim.width() - 1, dim.height() - 1);

    while let Some((Reverse(d), node)) = todo.pop() {
        let Node {
            pos: (x, y),
            dir,
            count,
        } = Node::unpack(dim, node);
        //eprintln!("({x},{y}) {dir:?}({count}) => {d}");
        if (x, y) == target && (!is_part2 || count >= 4) {
            return d;
        }
        update(&map, dir, (x, y), d, count, &mut seen, &mut todo);
    }
    panic!("Search failed")
}

pub fn update_part2(
    map: &Map,
    dir: Direction,
    (x, y): (usize, usize),
    d: usize,
    cnt: usize,
    seen: &mut BTreeSet<usize>,
    todo: &mut BinaryHeap<(Reverse<usize>, usize)>,
) {
    let dim = map.dim();
    if cnt >= 4 {
        if let Some((dir, (x1, y1))) = dim.turn_left(dir, (x, y)) {
            update(map, (x1, y1), d, dir, 1, seen, todo);
        }
        if let Some((dir, (x1, y1))) = dim.turn_right(dir, (x, y)) {
            update(map, (x1, y1), d, dir, 1, seen, todo);
        }
    }
    if cnt < 10 {
        if let Some((x1, y1)) = dim.go_straight(dir, (x, y)) {
            update(map, (x1, y1), d, dir, cnt + 1, seen, todo);
        }
    }
}

pub fn update_part1(
    map: &Map,
    dir: Direction,
    (x, y): (usize, usize),
    d: usize,
    cnt: usize,
    seen: &mut BTreeSet<usize>,
    todo: &mut BinaryHeap<(Reverse<usize>, usize)>,
) {
    let dim = map.dim();
    if let Some((dir, (x1, y1))) = dim.turn_left(dir, (x, y)) {
        update(map, (x1, y1), d, dir, 1, seen, todo);
    }
    if let Some((dir, (x1, y1))) = dim.turn_right(dir, (x, y)) {
        update(map, (x1, y1), d, dir, 1, seen, todo);
    }
    if cnt < 3 {
        if let Some((x1, y1)) = dim.go_straight(dir, (x, y)) {
            update(map, (x1, y1), d, dir, cnt + 1, seen, todo);
        }
    }
}

fn update(
    map: &Map,
    (x1, y1): (usize, usize),
    d: usize,
    dir: Direction,
    count: usize,
    seen: &mut BTreeSet<usize>,
    todo: &mut BinaryHeap<(Reverse<usize>, usize)>,
) {
    let w = map.ascii_num_at((x1, y1)) as usize;
    let p = Node::new((x1, y1), dir, count).pack(map.dim());
    let d2 = d + w; // New weight
    if seen.insert(p) {
        todo.push((Reverse(d2), p));
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run, Node};
    use crate::util::{Direction::Down, MapDimensions};

    #[test]
    fn test_pack() {
        let dim = MapDimensions::new(100, 100);
        let p = Node::new((27, 6), Down, 3).pack(&dim);
        assert_eq!(Node::unpack(&dim, p), Node::new((27, 6), Down, 3));
    }

    #[test]
    fn test_ord() {
        assert!((100, 20) > (99, 200));
    }

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day17/example.txt"), false), 102);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day17/input.txt"), false), 907);
    }

    #[test]
    fn example2() {
        assert_eq!(run(Path::new("res/day17/example.txt"), true), 94);
    }

    #[test]
    fn example2b() {
        assert_eq!(run(Path::new("res/day17/example2.txt"), true), 71);
    }

    #[test]
    fn part2() {
        assert_eq!(run(Path::new("res/day17/input.txt"), true), 1057);
    }
}
