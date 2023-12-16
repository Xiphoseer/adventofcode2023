use std::{collections::BTreeSet, path::Path};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FromDir {
    Top,
    Bottom,
    Left,
    Right,
}

struct ToDo {
    width: usize,
    height: usize,
    map: BTreeSet<(usize, usize, FromDir)>,
    visited: BTreeSet<(usize, FromDir)>,
}

type Enter = (usize, usize, FromDir);

impl ToDo {
    pub fn new(width: usize, height: usize, start: Enter) -> Self {
        Self {
            map: BTreeSet::from([start]),
            visited: BTreeSet::new(),
            width,
            height,
        }
    }

    pub fn pop(&mut self) -> Option<(usize, usize, FromDir)> {
        self.map.pop_first()
    }

    pub fn up(&mut self, x: usize, y: usize) {
        if y > 0 {
            self.map.insert((x, y - 1, FromDir::Bottom));
        }
    }

    pub fn down(&mut self, x: usize, y: usize) {
        if y + 1 < self.height {
            self.map.insert((x, y + 1, FromDir::Top));
        }
    }

    pub fn right(&mut self, x: usize, y: usize) {
        if x + 1 < self.width {
            self.map.insert((x + 1, y, FromDir::Left));
        }
    }

    pub fn left(&mut self, x: usize, y: usize) {
        if x > 0 {
            self.map.insert((x - 1, y, FromDir::Right));
        }
    }
}

pub fn run(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let width = text.split_once('\n').unwrap().0.len();
    let stride = width + 1;
    let height = text.len() / stride;

    energize(&text, height, stride, width, (0, 0, FromDir::Left))
}

pub fn run_part2(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let width = text.split_once('\n').unwrap().0.len();
    let stride = width + 1;
    let height = text.len() / stride;

    let mut max = 0;
    let mut start = (0, 0, FromDir::Left);

    for y in 0..height {
        let s = (0, y, FromDir::Left);
        let m = energize(&text, height, stride, width, s);
        if m > max {
            max = m;
            start = s;
        }
        let s = (width - 1, y, FromDir::Right);
        let m = energize(&text, height, stride, width, s);
        if m > max {
            max = m;
            start = s;
        }
    }
    for x in 0..width {
        let s = (x, 0, FromDir::Top);
        let m = energize(&text, height, stride, width, s);
        if m > max {
            max = m;
            start = s;
        }
        let s = (x, height - 1, FromDir::Bottom);
        let m = energize(&text, height, stride, width, s);
        if m > max {
            max = m;
            start = s;
        }
    }
    eprintln!("{start:?}");
    max
}

fn energize(text: &str, height: usize, stride: usize, width: usize, start: Enter) -> usize {
    let mut energized = vec![b'.'; text.len()];
    for y in 0..height {
        energized[y * stride + width] = b'\n';
    }
    let mut todo = ToDo::new(width, height, start);
    while let Some((x, y, from)) = todo.pop() {
        let index = y * stride + x;
        if todo.visited.insert((index, from)) {
            energized[index] = b'#';
            match (from, text.as_bytes()[index]) {
                (FromDir::Left | FromDir::Right, b'|') => {
                    todo.up(x, y);
                    todo.down(x, y);
                }
                (FromDir::Top | FromDir::Bottom, b'-') => {
                    todo.left(x, y);
                    todo.right(x, y);
                }
                (FromDir::Left, b'/')
                | (FromDir::Right, b'\\')
                | (FromDir::Bottom, b'.' | b'|') => todo.up(x, y),
                (FromDir::Right, b'/') | (FromDir::Left, b'\\') | (FromDir::Top, b'.' | b'|') => {
                    todo.down(x, y)
                }
                (FromDir::Top, b'/') | (FromDir::Bottom, b'\\') | (FromDir::Right, b'.' | b'-') => {
                    todo.left(x, y)
                }
                (FromDir::Bottom, b'/') | (FromDir::Top, b'\\') | (FromDir::Left, b'.' | b'-') => {
                    todo.right(x, y)
                }
                (_, c) => panic!("{c}"),
            }
        }
    }
    //std::io::Write::write_all(&mut std::io::stderr(), &energized).unwrap();
    energized.iter().copied().filter(|c| *c == b'#').count()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run, run_part2};

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day16/example.txt")), 46);
    }

    #[test]
    fn part1() {
        let e = run(Path::new("res/day16/input.txt"));
        assert!(e > 6930);
        assert_eq!(e, 8098);
    }

    #[test]
    fn example2() {
        assert_eq!(run_part2(Path::new("res/day16/example.txt")), 51);
    }

    #[test]
    fn part2() {
        assert_eq!(run_part2(Path::new("res/day16/input.txt")), 8335);
    }
}
