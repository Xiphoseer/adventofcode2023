use std::path::Path;

pub fn run(path: &Path, multiplier: usize) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let _width = text.chars().position(|x| x == '\n').unwrap();
    let _stride = _width + 1;
    let _height = text.len() / _stride;
    let _bytes = text.as_bytes();

    let blank_rows = text
        .lines()
        .enumerate()
        .filter_map(|(i, text)| text.chars().all(|c| c == '.').then_some(i))
        .collect::<Vec<_>>();

    let blank_cols = (0.._width)
        .filter(|x| (0.._height).all(|y| text.as_bytes()[y * _stride + x] == b'.'))
        .collect::<Vec<_>>();

    let galaxies = text
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| (b == b'#').then_some((i % _stride, i / _stride)))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        let (g1, rest) = galaxies[i..].split_first().unwrap();
        let col1 = blank_cols.binary_search(&g1.0).err().unwrap();
        let row1 = blank_rows.binary_search(&g1.1).err().unwrap();

        for g2 in rest {
            let col2 = blank_cols.binary_search(&g2.0).err().unwrap();
            let row2 = blank_rows.binary_search(&g2.1).err().unwrap();

            let xspaces = col1.abs_diff(col2);
            let yspaces = row1.abs_diff(row2);
            eprintln!("x{xspaces} y{yspaces}");

            let xdiff = g1.0.abs_diff(g2.0) + xspaces * (multiplier - 1);
            let ydiff = g1.1.abs_diff(g2.1) + yspaces * (multiplier - 1);

            let diff = xdiff + ydiff;
            eprintln!("{diff}");
            sum += diff;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day11/example.txt"), 2), 374);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day11/input.txt"), 2), 9177603);
    }

    #[test]
    fn example2() {
        assert_eq!(run(Path::new("res/day11/example.txt"), 10), 1030);
        assert_eq!(run(Path::new("res/day11/example.txt"), 100), 8410);
    }

    #[test]
    fn part2() {
        let len = run(Path::new("res/day11/input.txt"), 1000000);
        assert_ne!(len, 632004545607);
        assert_eq!(len, 632003913611);
    }
}
