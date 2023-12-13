use std::{ops::BitXor, path::Path};

pub fn run(path: &Path) -> usize {
    let patterns = decode(path);
    let mut sum = 0;
    for pat in patterns {
        for h in find_pairs(&pat.rows) {
            if check_reflection(&pat.rows, h) {
                sum += 100 * h;
            }
        }
        for v in find_pairs(&pat.cols) {
            if check_reflection(&pat.cols, v) {
                sum += v;
            }
        }
    }
    sum
}

pub fn run_part2(path: &Path) -> usize {
    let patterns = decode(path);
    let mut sum = 0;
    for pat in patterns {
        for h in 1..pat.rows.len() {
            if check_reflection_part2(&pat.rows, h) {
                sum += 100 * h;
            }
        }
        for v in 1..pat.cols.len() {
            if check_reflection_part2(&pat.cols, v) {
                sum += v;
            }
        }
    }
    sum
}

fn decode(path: &Path) -> Vec<Patterns> {
    let _text = std::fs::read_to_string(path).unwrap();
    let patterns = _text.split("\n\n").collect::<Vec<_>>();
    patterns
        .iter()
        .copied()
        .map(parse_line_pattern)
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Patterns {
    pub rows: Vec<usize>,
    pub cols: Vec<usize>,
}

pub fn find_pairs(scanlines: &[usize]) -> impl Iterator<Item = usize> + '_ {
    (1..scanlines.len()).filter(|x| scanlines[*x - 1] == scanlines[*x])
}

pub fn check_reflection_part2(scanlines: &[usize], index: usize) -> bool {
    let right = scanlines.len() - index;
    let space = index.min(right);
    let mut count_equal = 0;
    let mut count_one_off = 0;
    let mut count_rest = 0;
    for x in 0..space {
        let a = scanlines[index + x];
        let b = scanlines[index - 1 - x];
        if a == b {
            count_equal += 1;
        } else if a.bitxor(b).count_ones() == 1 {
            count_one_off += 1;
        } else {
            count_rest += 1;
        }
    }
    count_equal == space - 1 && count_one_off == 1 && count_rest == 0
}

pub fn check_reflection(scanlines: &[usize], index: usize) -> bool {
    let right = scanlines.len() - index;
    let space = index.min(right);
    (0..space).all(|x| scanlines[index + x] == scanlines[index - 1 - x])
}

fn parse_line_pattern(pat: &str) -> Patterns {
    Patterns {
        rows: pat.lines().map(parse_line).collect::<Vec<_>>(),
        cols: parse_col_patterns(pat),
    }
}

fn parse_col_patterns(pat: &str) -> Vec<usize> {
    let width = pat.split_once('\n').unwrap().0.len();
    let stride = width + 1;
    let height = (pat.len() + 1) / stride;
    let mut cols = Vec::with_capacity(width);
    for x in 0..width {
        let mut acc = 0;
        for y in 0..height {
            let ch = pat.split_at(y * stride + x).1.chars().next().unwrap();
            acc = acc << 1 | parse_char(ch);
        }
        cols.push(acc);
    }
    cols
}

fn parse_line(line: &str) -> usize {
    line.chars()
        .fold(0, |acc, next| acc << 1 | parse_char(next))
}

fn parse_char(next: char) -> usize {
    match next {
        '.' => 0, // ash
        '#' => 1, // rocks
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::day13::check_reflection;

    use super::{decode, run, run_part2};

    const EXAMPLE: &str = "res/day13/example.txt";

    #[test]
    fn example1() {
        let _pats = decode(Path::new(EXAMPLE));
        assert_eq!(
            _pats[0].rows,
            &[
                0b101100110, // #.##..##.
                0b001011010, // ..#.##.#.
                0b110000001, // ##......#
                0b110000001, // ##......#
                0b001011010, // ..#.##.#.
                0b001100110, // ..##..##.
                0b101011010, // #.#.##.#.
            ]
        );

        assert_eq!(
            _pats[0].cols, // same image, transposed
            &[
                0b1011001, 0b0011000, 0b1100111, 0b1000010, 0b0100101, 0b0100101, 0b1000010,
                0b1100111, 0b0011000,
            ]
        );

        let pairs = super::find_pairs(&_pats[0].cols).collect::<Vec<_>>();
        assert_eq!(pairs, &[5]);
        assert!(check_reflection(&_pats[0].cols, 5));
        assert!(!check_reflection(&_pats[0].cols, 6));

        let sum = run(Path::new(EXAMPLE));
        assert_eq!(sum, 405);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day13/input.txt")), 34821);
    }

    #[test]
    fn example2() {
        assert_eq!(run_part2(Path::new("res/day13/example.txt")), 400);
    }

    #[test]
    fn part2() {
        assert_eq!(run_part2(Path::new("res/day13/input.txt")), 36919);
    }
}
