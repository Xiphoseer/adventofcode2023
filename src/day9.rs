use std::path::Path;

pub fn run(path: &Path) -> Vec<Vec<isize>> {
    let _text = std::fs::read_to_string(path).unwrap();
    parse_sequences(_text)
}

pub fn predict_lists(lists: &[Vec<isize>], with: impl Fn(&[isize]) -> isize) -> Vec<isize> {
    lists.iter().map(Vec::as_ref).map(with).collect::<Vec<_>>()
}

fn parse_sequences(_text: String) -> Vec<Vec<isize>> {
    let lists = _text
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split(' ')
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    lists
}

pub fn predict_part1(entries: &[isize]) -> isize {
    let (first, rest) = entries.split_first().unwrap();
    let mut curr = *first;
    let mut derivative = vec![];
    let mut all_zeroes = true;
    for e in rest {
        let next = *e - curr;
        all_zeroes &= next == 0;
        derivative.push(next);
        curr = *e;
    }
    (if all_zeroes {
        0
    } else {
        predict_part1(&derivative)
    }) + *entries.last().unwrap()
}

pub fn predict_part2(entries: &[isize]) -> isize {
    let (first, rest) = entries.split_first().unwrap();
    let mut curr = *first;
    let mut derivative = vec![];
    let mut all_zeroes = true;
    for e in rest {
        let next = *e - curr;
        all_zeroes &= next == 0;
        derivative.push(next);
        curr = *e;
    }
    *entries.first().unwrap()
        - (if all_zeroes {
            0
        } else {
            predict_part2(&derivative)
        })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{predict_lists, predict_part1, predict_part2, run};

    #[test]
    fn example1() {
        let lists = run(Path::new("res/day9/example.txt"));
        let predictions = predict_lists(&lists, predict_part1);
        assert_eq!(predictions, vec![18, 28, 68]);
        assert_eq!(predictions.iter().sum::<isize>(), 114);
    }

    #[test]
    fn part1() {
        let lists = run(Path::new("res/day9/input.txt"));
        let predictions = predict_lists(&lists, predict_part1);
        assert_eq!(predictions.len(), 200);
        assert_eq!(predictions.iter().sum::<isize>(), 1762065988);
    }

    #[test]
    fn example2() {
        let lists = run(Path::new("res/day9/example.txt"));
        let predictions = predict_lists(&lists, predict_part2);
        assert_eq!(predictions, vec![-3, 0, 5]);
    }

    #[test]
    fn part2() {
        let lists = run(Path::new("res/day9/input.txt"));
        let predictions = predict_lists(&lists, predict_part2);
        assert_eq!(predictions.len(), 200);
        assert_eq!(predictions.iter().sum::<isize>(), 1066);
    }
}
