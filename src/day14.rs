use std::{
    collections::{btree_map::Entry, hash_map::DefaultHasher, BTreeMap},
    hash::{BuildHasher, BuildHasherDefault},
    mem,
    path::Path,
};

pub fn run_part1(path: &Path) -> usize {
    let _text = std::fs::read_to_string(path).unwrap();
    let width = _text.split_once('\n').unwrap().0.len();
    let stride = width + 1;
    let height = _text.len() / stride;
    let mut max = vec![height; width];

    let mut sum = 0;
    for (y, line) in _text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    max[x] = height - y - 1;
                }
                '.' => { /* do nothing */ }
                'O' => {
                    let next = max[x] - 1;
                    sum += mem::replace(&mut max[x], next);
                }
                _ => panic!("{c}"),
            }
        }
    }
    sum
}

pub struct Part2 {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
    stride: usize,
    bhash: BuildHasherDefault<DefaultHasher>,
}

impl Part2 {
    pub fn hash(&self) -> u64 {
        self.bhash.hash_one(&self.bytes)
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.bytes[y * self.stride + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.bytes[y * self.stride + x]
    }

    pub fn new(path: &Path) -> Self {
        let text = std::fs::read_to_string(path).unwrap();
        let width = text.split_once('\n').unwrap().0.len();
        let stride = width + 1;
        let height = text.len() / stride;
        Self {
            bytes: text.clone().into_bytes(),
            width,
            height,
            stride,
            bhash: BuildHasherDefault::<DefaultHasher>::default(),
        }
    }
}

impl Part2 {
    pub fn find_loop(&mut self) -> usize {
        let _h0 = self.hash();
        let mut set = BTreeMap::new();
        let mut count = 0usize;
        let prev_index = loop {
            self.cycle();
            count += 1;
            let hash = self.hash();
            match set.entry(hash) {
                Entry::Vacant(v) => {
                    v.insert(count);
                }
                Entry::Occupied(o) => {
                    let prev_index = *o.get();
                    break prev_index;
                }
            };
        };
        let cycle = count - prev_index;
        let rest = 1000000000 - count;
        let rem: usize = rest % cycle;
        eprintln!("{prev_index} + n x {cycle}; {rest}; {rem}");
        for _ in 0..rem {
            self.cycle();
        }
        self.load()
    }

    pub fn cycle(&mut self) -> (u64, u64, u64, u64) {
        self.step_north();
        let h1 = self.hash();
        self.step_west();
        let h2 = self.hash();
        self.step_south();
        let h3 = self.hash();
        self.step_east();
        let h4 = self.hash();
        (h1, h2, h3, h4)
    }

    pub fn load(&self) -> usize {
        self.bytes
            .chunks(self.stride)
            .map(|a| &a[..self.width])
            .enumerate()
            .map(|(y, line)| line.iter().filter(|&x| *x == b'O').count() * (self.height - y))
            .sum()
    }

    pub fn step_north(&mut self) {
        let mut min = vec![0; self.width];
        for y in 0..self.height {
            for (x, m) in min.iter_mut().enumerate() {
                match self.get(x, y) {
                    b'#' => {
                        *m = y + 1;
                    }
                    b'.' => { /* do nothing */ }
                    b'O' => {
                        let prev = *m;
                        let next = prev + 1;
                        *self.get_mut(x, y) = b'.';
                        *self.get_mut(x, prev) = b'O';
                        *m = next;
                    }
                    c => panic!("{c}"),
                }
            }
        }
    }

    pub fn step_west(&mut self) {
        let mut min = vec![0; self.height];
        for x in 0..self.width {
            for (y, m) in min.iter_mut().enumerate() {
                match self.get(x, y) {
                    b'#' => {
                        *m = x + 1;
                    }
                    b'.' => { /* do nothing */ }
                    b'O' => {
                        let prev = *m;
                        let next = prev + 1;
                        *self.get_mut(x, y) = b'.';
                        *self.get_mut(prev, y) = b'O';
                        *m = next;
                    }
                    c => panic!("{c}"),
                }
            }
        }
    }

    pub fn step_south(&mut self) {
        let mut max = vec![self.height; self.width];
        for y in (0..self.height).rev() {
            for (x, m) in max.iter_mut().enumerate() {
                match self.get(x, y) {
                    b'#' => {
                        *m = y;
                    }
                    b'.' => { /* do nothing */ }
                    b'O' => {
                        let prev = *m;
                        let next = prev - 1;
                        *self.get_mut(x, y) = b'.';
                        *self.get_mut(x, next) = b'O';
                        *m = next;
                    }
                    c => panic!("{c}"),
                }
            }
        }
    }

    pub fn step_east(&mut self) {
        let mut max = vec![self.width; self.height];
        for x in (0..self.width).rev() {
            for (y, m) in max.iter_mut().enumerate() {
                match self.get(x, y) {
                    b'#' => {
                        *m = x;
                    }
                    b'.' => { /* do nothing */ }
                    b'O' => {
                        let prev = *m;
                        let next = prev - 1;
                        *self.get_mut(x, y) = b'.';
                        *self.get_mut(next, y) = b'O';
                        *m = next;
                    }
                    c => panic!("{c}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run_part1, Part2};

    #[test]
    fn example1() {
        assert_eq!(run_part1(Path::new("res/day14/example.txt")), 136);
    }

    #[test]
    fn part1() {
        assert_eq!(run_part1(Path::new("res/day14/input.txt")), 105784);
    }

    #[test]
    fn example2() {
        let map1 = std::fs::read("res/day14/cycle1.txt").unwrap();
        let map2 = std::fs::read("res/day14/cycle2.txt").unwrap();
        let map3 = std::fs::read("res/day14/cycle3.txt").unwrap();

        let path = Path::new("res/day14/example.txt");
        let mut map = Part2::new(path);
        map.cycle();
        assert_eq!(&map.bytes, &map1);
        map.cycle();
        assert_eq!(&map.bytes, &map2);
        map.cycle();
        assert_eq!(&map.bytes, &map3);

        let mut map = Part2::new(path);
        let load = map.find_loop();
        assert_eq!(load, 64);
    }

    #[test]
    fn part2() {
        let mut map = Part2::new(Path::new("res/day14/input.txt"));
        let load = map.find_loop();
        assert_eq!(load, 91286);
    }
}
