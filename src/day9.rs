use std::{mem, path::Path};

pub fn run(path: &Path) -> Vec<Vec<isize>> {
    std::fs::read_to_string(path).map(parse_sequences).unwrap()
}

fn parse_sequences(text: String) -> Vec<Vec<isize>> {
    text.lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.split(' ').map(|i| i.parse().unwrap()).collect())
        .collect()
}

pub fn predict_lists(lists: &mut [Vec<isize>], with: impl Fn(&mut [isize]) -> isize) -> Vec<isize> {
    lists.iter_mut().map(Vec::as_mut_slice).map(with).collect()
}

pub fn predict_next(entries: &mut [isize]) -> isize {
    let (first, derive) = entries.split_first_mut().unwrap();
    let mut all_zeroes = true;
    derive.iter_mut().fold(*first, |curr, e| {
        let next = *e - curr;
        all_zeroes &= next == 0;
        mem::replace(e, next)
    }) + if all_zeroes { 0 } else { predict_next(derive) }
}

pub fn predict_prev(entries: &mut [isize]) -> isize {
    let (last, derive) = entries.split_last_mut().unwrap();
    let mut all_zeroes = true;
    derive.iter_mut().rev().fold(*last, |curr, e| {
        let next = curr - *e;
        all_zeroes &= next == 0;
        mem::replace(e, next)
    }) - if all_zeroes { 0 } else { predict_prev(derive) }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{predict_lists, predict_next, predict_prev, run};

    #[test]
    fn example1() {
        let mut lists = run(Path::new("res/day9/example.txt"));
        let predictions = predict_lists(&mut lists, predict_next);
        assert_eq!(predictions, vec![18, 28, 68]);
        assert_eq!(predictions.iter().sum::<isize>(), 114);
    }

    #[test]
    fn part1() {
        let mut lists = run(Path::new("res/day9/input.txt"));
        let predictions = predict_lists(&mut lists, predict_next);
        assert_eq!(predictions.len(), 200);
        assert_eq!(predictions.iter().sum::<isize>(), 1762065988);
    }

    #[test]
    fn example2() {
        let mut lists = run(Path::new("res/day9/example.txt"));
        let predictions = predict_lists(&mut lists, predict_prev);
        assert_eq!(predictions, vec![-3, 0, 5]);
    }

    #[test]
    fn part2() {
        let mut lists = run(Path::new("res/day9/input.txt"));
        let predictions = predict_lists(&mut lists, predict_prev);
        assert_eq!(predictions.len(), 200);
        assert_eq!(predictions.iter().sum::<isize>(), 1066);
    }
}
