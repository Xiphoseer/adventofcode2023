use std::{cmp::Ordering, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::{many1, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

pub struct RangeMap {
    entries: Vec<(usize, usize, usize)>,
}

impl RangeMap {
    fn new(mut entries: Vec<(usize, usize, usize)>) -> Self {
        entries.sort_by_key(|(_a, src_start, _c)| *src_start);
        Self { entries }
    }
}

pub struct Input {
    pub seeds: Vec<usize>,
    pub maps: Vec<RangeMap>,
}

impl Input {
    fn new(seeds: Vec<usize>, maps: Vec<RangeMap>) -> Input {
        Input { seeds, maps }
    }

    pub fn transform(&self, seed: usize) -> usize {
        let mut v = seed;
        for (_i, map) in self.maps.iter().enumerate() {
            v = map.transform(v);
        }
        v
    }

    pub fn transform_debug(&self, seed: usize) -> Vec<usize> {
        let mut v = seed;
        let mut steps = vec![v];
        for (_i, map) in self.maps.iter().enumerate() {
            v = map.transform(v);
            steps.push(v);
        }
        steps
    }

    pub fn lowest_location(&self) -> Option<usize> {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.transform(seed))
            .min()
    }

    pub fn lowest_location_part2(&self) -> Option<usize> {
        self.seeds
            .chunks_exact(2)
            .map(|range| {
                let start = range[0];
                let len = range[1];
                (start..=(start + len))
                    .map(|seed| self.transform(seed))
                    .min()
                    .unwrap()
            })
            .min()
    }
}

impl RangeMap {
    fn transform(&self, v: usize) -> usize {
        if let Ok(index) = self.entries.binary_search_by(|(_, src_start, len)| {
            if *src_start > v {
                Ordering::Greater
            } else if v >= src_start + len {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            let (dest_start, src_start, _len) = self.entries[index];
            v + dest_start - src_start
        } else {
            v
        }
    }
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, <usize as FromStr>::from_str)(input)
}

pub fn parse_input_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let p2 = separated_list0(tag(" "), parse_num);
    let mut p3 = preceded(tag("seeds: "), p2);
    let (input, seeds) = p3(input)?;
    Ok((input, seeds))
}

fn parse_space_num(input: &str) -> IResult<&str, usize> {
    preceded(tag(" "), parse_num)(input)
}

pub fn parse_triple(input: &str) -> IResult<&str, (usize, usize, usize)> {
    tuple((parse_num, parse_space_num, parse_space_num))(input)
}

pub fn parse_map_title(input: &str) -> IResult<&str, (&str, &str)> {
    pair(
        terminated(alpha1, tag("-to-")),
        terminated(alpha1, tag(" map:")),
    )(input)
}

pub fn parse_map(input: &str) -> IResult<&str, RangeMap> {
    map(
        preceded(parse_map_title, many1(preceded(tag("\n"), parse_triple))),
        RangeMap::new,
    )(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        pair(parse_input_seeds, many1(preceded(tag("\n\n"), parse_map))),
        |(a, b)| Input::new(a, b),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::parse_input_seeds;
    use super::parse_triple;

    #[test]
    fn parse_seed_line() {
        let (_rest, seeds) = parse_input_seeds("seeds: 79 14 55 13").unwrap();
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn parse_triples() {
        let (_rest, seeds) = parse_triple("0 15 37").unwrap();
        assert_eq!(seeds, (0, 15, 37));
    }

    #[test]
    fn parse_name() {
        let (_rest, names) = super::parse_map_title("soil-to-fertilizer map:").unwrap();
        assert_eq!(names, ("soil", "fertilizer"))
    }

    #[test]
    fn parse_map() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let (_rest, map) = super::parse_map(text).unwrap();
        assert_eq!(map.entries, vec![(52, 50, 48), (50, 98, 2),])
    }

    #[test]
    fn parse_example() {
        let text = std::fs::read_to_string("res/day5/example.txt").unwrap();
        let (_rest, input) = super::parse_input(&text).unwrap();
        assert_eq!(&input.seeds, &[79, 14, 55, 13]);
        assert_eq!(input.maps.len(), 7);

        assert_eq!(input.seeds[0], 79);
        assert_eq!(
            input.transform_debug(79),
            vec![79, 81, 81, 81, 74, 78, 78, 82]
        );
        assert_eq!(input.transform(input.seeds[1]), 43);
        assert_eq!(input.transform(input.seeds[2]), 86);
        assert_eq!(input.transform(input.seeds[3]), 35);

        assert_eq!(input.lowest_location(), Some(35));
        assert_eq!(input.lowest_location_part2(), Some(46));
    }

    #[test]
    fn part1() {
        let text = std::fs::read_to_string("res/day5/input.txt").unwrap();
        let (_rest, input) = super::parse_input(&text).unwrap();
        assert_eq!(input.maps.len(), 7);

        assert_eq!(input.lowest_location(), Some(51752125));
        assert_eq!(input.lowest_location_part2(), Some(12634632));
    }
}
