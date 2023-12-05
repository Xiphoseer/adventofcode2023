use std::{cmp::Ordering, ops::Range, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::{many1, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEntry {
    dest_start: usize,
    src: Range<usize>,
}

impl From<(usize, usize, usize)> for MapEntry {
    fn from((dest_start, src_start, len): (usize, usize, usize)) -> Self {
        Self {
            dest_start,
            src: src_start..(src_start + len),
        }
    }
}

pub struct RangeMap {
    entries: Vec<MapEntry>,
}

impl RangeMap {
    fn new(mut entries: Vec<MapEntry>) -> Self {
        entries.sort_by_key(|e| e.src.start);
        Self { entries }
    }

    pub fn transform_list(&self, ranges: &mut [Range<usize>]) -> Vec<Range<usize>> {
        ranges.sort_by_key(|r| r.start);
        let mut map_entries = self.entries.iter().cloned().peekable();
        let mut result = vec![];
        for mut range in ranges.iter().cloned() {
            loop {
                if let Some(next_map_entry) = map_entries.peek() {
                    if next_map_entry.src.end <= range.start {
                        // Map Entry before Range, pop and skip to next entry
                        map_entries.next();
                        continue;
                    } else if next_map_entry.src.start >= range.end {
                        // Map Entry after Range, keep in queue and transfer and next range
                        result.push(range.clone());
                        break;
                    } else if next_map_entry.src.start <= range.start {
                        // Range starts within entry
                        let start =
                            range.start + next_map_entry.dest_start - next_map_entry.src.start;
                        if range.end <= next_map_entry.src.end {
                            // Range fully in map entry
                            let end =
                                range.end + next_map_entry.dest_start - next_map_entry.src.start;
                            // Consume and keep entry in iter
                            result.push(start..end);
                            break;
                        } else {
                            // Range ends after entry
                            let end = next_map_entry.src.end + next_map_entry.dest_start
                                - next_map_entry.src.start;
                            // Consume and keep entry in iter
                            result.push(start..end);
                            // Reset range start
                            range.start = next_map_entry.src.end;
                            // Do next range
                            continue;
                        };
                    } else {
                        // Range starts before map entry
                        assert!(range.start < next_map_entry.src.start);
                        // Push range before map entry
                        result.push(range.start..next_map_entry.src.start);

                        if range.end <= next_map_entry.src.end {
                            // Range starts before but ends within range
                            range.start = next_map_entry.src.start;
                            continue; // do here?
                        } else {
                            let start = next_map_entry.dest_start;
                            let end = next_map_entry.dest_start + next_map_entry.src.len();
                            range.start = next_map_entry.src.end;
                            map_entries.next();
                            result.push(start..end);
                            continue;
                        }
                    }
                } else {
                    // If there is no next map range, push the full range
                    result.push(range.clone());
                    break;
                }
            }
        }
        result
    }

    fn transform(&self, v: usize) -> usize {
        if let Ok(index) = self.entries.binary_search_by(|e| {
            if e.src.start > v {
                Ordering::Greater
            } else if v >= e.src.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            let e = &self.entries[index];
            v + e.dest_start - e.src.start
        } else {
            v
        }
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

    pub fn ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        self.seeds.chunks_exact(2).map(|range| {
            let start = range[0];
            let len = range[1];
            start..(start + len)
        })
    }

    pub fn lowest_location_part2_alt(&self) -> Option<usize> {
        let mut ranges: Vec<_> = self.ranges().collect();
        for map in &self.maps {
            ranges = map.transform_list(&mut ranges);
        }
        ranges.iter().map(|x| x.start).min()
    }

    pub fn lowest_location_part2(&self) -> Option<usize> {
        self.ranges()
            .map(|range| range.map(|seed| self.transform(seed)).min().unwrap())
            .min()
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

pub fn parse_triple(input: &str) -> IResult<&str, MapEntry> {
    map(
        tuple((parse_num, parse_space_num, parse_space_num)),
        MapEntry::from,
    )(input)
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
    use crate::day5::MapEntry;

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
        assert_eq!(
            seeds,
            MapEntry {
                dest_start: 0,
                src: 15..52
            }
        );
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
        assert_eq!(
            map.entries,
            vec![
                MapEntry {
                    dest_start: 52,
                    src: 50..98,
                },
                MapEntry {
                    dest_start: 50,
                    src: 98..100,
                },
            ]
        )
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
    }

    #[test]
    fn part2_example() {
        let text = std::fs::read_to_string("res/day5/example.txt").unwrap();
        let (_rest, input) = super::parse_input(&text).unwrap();
        assert_eq!(input.lowest_location_part2(), Some(46));
        assert_eq!(input.lowest_location_part2_alt(), Some(46));
    }

    #[test]
    fn part2() {
        let text = std::fs::read_to_string("res/day5/input.txt").unwrap();
        let (_rest, input) = super::parse_input(&text).unwrap();
        assert_eq!(input.lowest_location_part2_alt(), Some(12634632));
        //assert_eq!(input.lowest_location_part2(), Some(12634632));
    }
}
