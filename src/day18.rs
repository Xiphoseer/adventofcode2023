use std::path::Path;

use regex::Regex;

use crate::util::{
    area::{area, Edge, EdgeMap, Noop},
    Direction,
};

pub fn parse_input_part1(text: &str) -> Vec<(Direction, usize)> {
    let regex = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-f]{6})\)$").unwrap();
    text.lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let c = regex.captures(line).unwrap();
            let dir = match c.get(1).unwrap().as_str() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            let len: usize = c.get(2).unwrap().as_str().parse().unwrap();
            //let color_hex = c.get(3).unwrap().as_str();
            //let color = u32::from_str_radix(color_hex, 16).unwrap();
            (dir, len)
        })
        .collect::<Vec<_>>()
}

pub fn parse_input_part2(text: &str) -> Vec<(Direction, usize)> {
    let regex = Regex::new(r"^([UDLR]) (\d+) \(#([0-9a-f]{6})\)$").unwrap();
    text.lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let c = regex.captures(line).unwrap();
            let hex_str = c.get(3).unwrap().as_str();
            let len = usize::from_str_radix(&hex_str[..5], 16).unwrap();
            let dir = match hex_str.as_bytes()[5] {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!(),
            };
            (dir, len)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Turn {
    Right,
    Left,
}

pub fn run_part2(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let input = parse_input_part2(&text);

    let mut turns = Vec::with_capacity(input.len());
    let mut prev_dir = input.last().unwrap().0;
    for (dir, count) in input {
        use Direction::*;
        let turn = match (prev_dir, dir) {
            (Right, Up) | (Up, Left) | (Left, Down) | (Down, Right) => Turn::Left,
            (Right, Down) | (Up, Right) | (Left, Up) | (Down, Left) => Turn::Right,
            _ => panic!("{prev_dir:?}=>{dir:?}"),
        };
        turns.push((turn, count));
        prev_dir = dir;
    }
    let (left_turns, right_turns) = turns.iter().fold((0, 0), |(l, r), (turn, _)| match turn {
        Turn::Left => (l + 1, r),
        Turn::Right => (l, r + 1),
    });

    let (turn_in, turn_out) = if left_turns > right_turns {
        (Turn::Left, Turn::Right)
    } else {
        (Turn::Right, Turn::Left)
    };

    let is_out_in_in_gt = |slice: &[(Turn, usize)]| {
        (slice[0].0, slice[1].0, slice[2].0) == (turn_out, turn_in, turn_in)
            && slice[2].1 > slice[0].1
    };

    let is_in_out_out_gt = |slice: &[(Turn, usize)]| {
        (slice[0].0, slice[1].0, slice[2].0) == (turn_in, turn_out, turn_out)
            && slice[2].1 > slice[0].1
    };

    let is_in_out_in = |slice: &[(Turn, usize)]| {
        (slice[0].0, slice[1].0, slice[2].0) == (turn_in, turn_out, turn_in)
    };

    let is_out_in_out = |slice: &[(Turn, usize)]| {
        (slice[0].0, slice[1].0, slice[2].0) == (turn_out, turn_in, turn_out)
    };

    let mut balance = 0isize;

    while turns.len() > 4 {
        eprintln!("1: {turns:?}");
        while let Some(next) = find_next(&turns, is_in_out_in) {
            let (b, c) = update1(&mut turns, next); // the resulting shape is too big
            balance -= (b * c) as isize;
        }

        eprintln!("2: {turns:?}");
        while let Some(next) = find_next(&turns, is_out_in_in_gt) {
            let (b, c) = update2(&mut turns, next); // the resulting shape is too small;
            balance += (b * (c + 1)) as isize;
        }

        eprintln!("3: {turns:?}");
        while let Some(next) = find_next(&turns, is_out_in_out) {
            let (b, c) = update1(&mut turns, next); // the resulting shape is too small;
            balance += (b * c) as isize;
        }

        eprintln!("4: {turns:?}");
        while let Some(next) = find_next(&turns, is_in_out_out_gt) {
            let (b, c) = update2(&mut turns, next); // the resulting shape is too big
            balance -= (b * (c - 1)) as isize;
        }
        eprintln!("balance: {balance}");
    }
    eprintln!("end: {turns:?}");

    assert!(turns.iter().copied().all(|(turn, _)| turn == turn_in));
    assert_eq!(turns[0].1, turns[2].1);
    assert_eq!(turns[1].1, turns[3].1);

    balance += ((turns[0].1 + 1) * (turns[1].1 + 1)) as isize;
    balance as usize
}

fn update1(turns: &mut Vec<(Turn, usize)>, next: usize) -> (usize, usize) {
    let (a, b, c, d) = get_indices(turns, next);
    let diff = (turns[b].1, turns[c].1);
    turns[a].1 += turns[c].1;
    turns[d].1 += turns[b].1;
    remove_two(turns, b, c);
    diff
}

fn remove_two(turns: &mut Vec<(Turn, usize)>, b: usize, c: usize) {
    if b > c {
        turns.remove(b);
        turns.remove(c);
    } else {
        turns.remove(c);
        turns.remove(b);
    }
}

fn update2(turns: &mut Vec<(Turn, usize)>, next: usize) -> (usize, usize) {
    let (a, b, c, d) = get_indices(turns, next);
    let diff = (turns[b].1, turns[c].1);
    turns[a].1 += turns[c].1;
    turns[d].1 -= turns[b].1;
    remove_two(turns, b, c);
    diff
}

fn get_indices(turns: &mut Vec<(Turn, usize)>, next: usize) -> (usize, usize, usize, usize) {
    let len = turns.len();
    let a = (next + (len - 1)) % len;
    let b = next;
    let c = (next + 1) % len;
    let d = (next + 2) % len;
    (a, b, c, d)
}

fn find_next(
    turns: &[(Turn, usize)],
    is_out_in_in: impl Fn(&[(Turn, usize)]) -> bool,
) -> Option<usize> {
    let len = turns.len();
    turns
        .windows(3)
        .chain({
            let first = turns[0];
            let second = turns[1];
            let second_to_last = turns[len - 2];
            let last = turns[len - 1];
            [
                &[second_to_last, last, first][..],
                &[last, first, second][..],
            ]
        })
        .position(is_out_in_in)
}

pub fn run_part1(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let input = parse_input_part1(&text);

    let mut edges = EdgeMap::<isize>::new();
    let mut pos = (0isize, 0isize);
    let mut prev_dir = input.last().map(|f| f.0).unwrap();
    for (dir, cnt) in input {
        edges.insert(pos, Edge::of_dir_pair(prev_dir, dir).unwrap());
        match dir {
            Direction::Right => {
                let edge = Edge::EastWest;
                for i in 0..cnt {
                    pos = (pos.0 + 1, pos.1);
                    if i + 1 < cnt {
                        edges.insert(pos, edge);
                    }
                }
            }
            Direction::Up => {
                let edge = Edge::NorthSouth;
                for i in 0..cnt {
                    pos = (pos.0, pos.1 - 1);
                    if i + 1 < cnt {
                        edges.insert(pos, edge);
                    }
                }
            }
            Direction::Left => {
                let edge = Edge::EastWest;
                for i in 0..cnt {
                    pos = (pos.0 - 1, pos.1);
                    if i + 1 < cnt {
                        edges.insert(pos, edge);
                    }
                }
            }
            Direction::Down => {
                let edge = Edge::NorthSouth;
                for i in 0..cnt {
                    pos = (pos.0, pos.1 + 1);
                    if i + 1 < cnt {
                        edges.insert(pos, edge);
                    }
                }
            }
        }
        prev_dir = dir;
    }

    let min = edges
        .keys()
        .copied()
        .fold((0, 0), |(ax, ay), (bx, by)| (ax.min(bx), ay.min(by)));

    let max = edges
        .keys()
        .copied()
        .fold((0, 0), |(ax, ay), (bx, by)| (ax.max(bx), ay.max(by)));

    let count_inner = area(min.0..max.0, min.1..max.1, &edges, &mut Noop);

    count_inner + edges.len()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run_part1, run_part2};

    #[test]
    fn example1() {
        assert_eq!(run_part1(Path::new("res/day18/example.txt")), 62);
    }

    #[test]
    fn part1() {
        let count = run_part1(Path::new("res/day18/input.txt"));
        assert_ne!(count, 42590);
        assert_eq!(count, 42317);
    }

    #[test]
    fn example2() {
        let count = run_part2(Path::new("res/day18/example.txt"));
        assert_eq!(count, 952408144115);
    }

    #[test]
    fn part2() {
        let count = run_part2(Path::new("res/day18/input.txt"));
        assert_ne!(count, 83605485349422);
        assert!(count > 83605514877556, "answer too low");
        assert_eq!(count, 83605563360288);
    }
}
