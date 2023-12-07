use std::f32::EPSILON;

/// d = s * (t - s)
///
/// => -s^2 + ts = d
/// => s^2 - ts = -d
/// => s^2 - ts + d = 0
///
/// p = -t
/// q = d
///
/// x0 = t/2 +/- sqrt((-t/2)^2 - d)
fn solve_for_s(t: f32, d: f32) -> Option<(f32, f32)> {
    let t2 = t / 2.0;
    let in_root = (t * t) / 4.0 - d;
    if in_root > 0.0 {
        let root = in_root.sqrt();
        Some(((t2 - root), (t2 + root)))
    } else {
        None
    }
}

pub fn min_max(t: usize, d: usize) -> Option<(usize, usize)> {
    let e = 10.0 * EPSILON;
    solve_for_s(t as f32, d as f32)
        .map(|(s_min, s_max)| ((s_min + e).ceil() as usize, (s_max - e).floor() as usize))
}

pub fn num_ways_to_win(t: usize, d: usize) -> usize {
    if let Some((min, max)) = min_max(t, d) {
        if max >= min {
            return max + 1 - min;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day6::{min_max, num_ways_to_win};

    use super::solve_for_s;

    #[test]
    fn example() {
        assert_eq!(solve_for_s(7.0, 9.0), Some((1.6972244, 5.3027754)));
        assert_eq!(num_ways_to_win(7, 9), 4);
        assert_eq!(num_ways_to_win(15, 40), 8);
        assert_eq!(solve_for_s(30.0, 200.0), Some((10.0, 20.0)));
        assert_eq!(num_ways_to_win(30, 200), 9);
    }

    #[test]
    fn part1() {
        let text = std::fs::read_to_string("res/day6/input.txt").unwrap();
        let mut lines = text.lines();
        let times = parse_input_line(lines.next().unwrap(), "Time:");
        let distances = parse_input_line(lines.next().unwrap(), "Distance:");
        let input = times.zip(distances).collect::<Vec<_>>();

        let solved = input
            .iter()
            .map(|(t, d)| solve_for_s(*t as f32, *d as f32))
            .collect::<Vec<_>>();
        assert_eq!(
            solved,
            &[
                Some((5.6691, 51.330902)),
                Some((24.864471, 47.13553)),
                Some((30.725082, 38.274918)),
                Some((36.51317, 55.48683)),
            ]
        );

        let min_max_results = input
            .iter()
            .map(|(t, d)| min_max(*t, *d))
            .collect::<Vec<_>>();

        assert_eq!(
            min_max_results,
            &[
                Some((6, 51)),
                Some((25, 47)),
                Some((31, 38)),
                Some((37, 55)),
            ]
        );

        let mut product = 1usize;
        for (t, d) in input {
            product *= num_ways_to_win(t, d);
        }
        assert_eq!(product, 160816);
    }

    fn parse_input_line<'a>(
        line: &'a str,
        prefix: &'static str,
    ) -> impl Iterator<Item = usize> + 'a {
        line.strip_prefix(prefix)
            .unwrap()
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|f| f.trim().parse::<usize>().unwrap())
    }

    #[test]
    fn part2() {
        let t = 57726992;
        let d = 291117211762026;
        assert_eq!(num_ways_to_win(t, d), 46561107);
    }
}
