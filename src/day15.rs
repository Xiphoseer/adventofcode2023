use std::path::Path;

pub fn run(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let text = text.trim_end();
    text.split(',').map(hash).map(usize::from).sum()
}

pub fn hash(input: &str) -> u8 {
    input
        .bytes()
        .fold(0, |a, b| a.wrapping_add(b).wrapping_mul(17))
}

pub fn run_part2(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let text = text.trim_end();
    let mut map: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_i| vec![]);
    for cmd in text.split(',') {
        if let Some(label) = cmd.strip_suffix('-') {
            let _box = hash(label);
            let _box = &mut map[_box as usize];
            if let Some(pos) = _box.iter().position(|x| x.0 == label) {
                _box.remove(pos);
            }
        } else {
            let (label, focal_length) = cmd.split_once('=').unwrap();
            let _box = hash(label);
            let _box = &mut map[_box as usize];
            let focal_length: u8 = focal_length.parse().unwrap();
            if let Some((_, f)) = _box.iter_mut().find(|x| x.0 == label) {
                *f = focal_length;
            } else {
                _box.push((label, focal_length));
            }
        }
    }
    eprintln!("{:?}", map);
    map.iter()
        .enumerate()
        .map(|(bi, contents)| {
            (bi + 1)
                * contents
                    .iter()
                    .enumerate()
                    .map(|(i, (_, f))| (i + 1) * (*f as usize))
                    .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run, run_part2};

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day15/example.txt")), 1320);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day15/input.txt")), 503154);
    }

    #[test]
    fn example2() {
        assert_eq!(run_part2(Path::new("res/day15/example.txt")), 145);
    }

    #[test]
    fn part2() {
        assert_eq!(run_part2(Path::new("res/day15/input.txt")), 251353);
    }
}
