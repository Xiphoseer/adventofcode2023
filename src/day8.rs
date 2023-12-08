use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    path::Path,
    str::FromStr,
};

use num::integer::lcm;
use regex::Regex;

pub enum Dir {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateID(u32);

impl StateID {
    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                &self.0 as *const u32 as *const u8,
                3,
            ))
        }
    }

    pub fn is_end(&self) -> bool {
        const Z: u32 = u32::from_ne_bytes([0, 0, b'Z', 0]);
        const MASK: u32 = u32::from_ne_bytes([0, 0, 0xFF, 0]);
        self.0 & MASK == Z
    }
}

impl FromStr for StateID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match *s.as_bytes() {
            [a, b, c] if a.is_ascii() && b.is_ascii() && c.is_ascii() => {
                Ok(Self(u32::from_ne_bytes([a, b, c, 0])))
            }
            _ => Err(()),
        }
    }
}

impl fmt::Debug for StateID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub left: StateID,
    pub right: StateID,
}

pub struct Data {
    steps: Vec<Dir>,
    transitions: BTreeMap<StateID, State>,
    start: BTreeSet<StateID>,
    end: BTreeSet<StateID>,
}

pub fn run(path: &Path) -> Data {
    let _text = std::fs::read_to_string(path).unwrap();
    let mut lines = _text.lines();

    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    lines.next();

    let mut start = BTreeSet::<StateID>::new();
    let mut end = BTreeSet::<StateID>::new();
    let regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let transitions = lines
        .map(|l| {
            let cap = regex.captures(l).unwrap();
            let name = cap.get(1).unwrap().as_str();
            let name_id = name.parse().unwrap();
            if name.ends_with('A') {
                start.insert(name_id);
            } else if name.ends_with('Z') {
                end.insert(name_id);
            }
            let left = cap.get(2).unwrap().as_str();
            let right = cap.get(3).unwrap().as_str();
            (
                name_id,
                State {
                    left: left.parse().unwrap(),
                    right: right.parse().unwrap(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    Data {
        steps,
        transitions,
        start,
        end,
    }
}

pub fn part1(path: &Path) -> usize {
    let data = run(path);
    let end = "ZZZ".parse().unwrap();
    let mut state = "AAA".parse().unwrap();
    let mut dir = data.steps.iter().cycle();
    let mut count = 0;
    while state != end {
        let s = data.transitions.get(&state).unwrap();
        state = match dir.next().unwrap() {
            Dir::Left => s.left,
            Dir::Right => s.right,
        };
        count += 1;
    }
    count
}

/*#[derive(Debug)]
pub struct CycleInfo {
    count: usize,
    ends: BTreeSet<usize>,
    end_syms: BTreeSet<usize>,
}*/

fn c(s: &str) -> StateID {
    s.parse().unwrap()
}

pub struct StateIter<'a> {
    next: StateID,
    t: &'a BTreeMap<StateID, StateID>,
}

impl<'a> StateIter<'a> {
    pub fn new(t: &'a BTreeMap<StateID, StateID>, start: StateID) -> Self {
        Self { t, next: start }
    }
}

impl Iterator for StateIter<'_> {
    type Item = StateID;

    fn next(&mut self) -> Option<Self::Item> {
        let then = *self.t.get(&self.next)?;
        self.next = then;
        Some(then)
    }
}

pub fn part2_special_case(path: &Path) -> usize {
    let data = run(path);

    let steps_per_cycle = data.steps.len();
    assert_eq!(steps_per_cycle, 281);

    let starts_check: [StateID; 6] = [c("AAA"), c("NFA"), c("LJA"), c("PLA"), c("KTA"), c("JXA")];
    assert_eq!(data.start, BTreeSet::from(starts_check));

    let end_check: [StateID; 6] = [c("HBZ"), c("BGZ"), c("RGZ"), c("DLZ"), c("NTZ"), c("ZZZ")];
    assert_eq!(data.end, BTreeSet::from(end_check));

    let mut ends = BTreeMap::<StateID, Vec<(usize, StateID)>>::new();
    let mut cycle_state = BTreeMap::<StateID, StateID>::new();

    for s in data.transitions.keys() {
        let start_sym = *s;
        let mut state = start_sym;

        for (index, step) in data.steps.iter().enumerate() {
            let transition = data.transitions.get(&state).unwrap();
            state = match *step {
                Dir::Left => transition.left,
                Dir::Right => transition.right,
            };
            if state.is_end() {
                ends.entry(start_sym).or_default().push((index, state));
            }
        }

        cycle_state.insert(start_sym, state);
    }

    // Note how all of them are at index 280, so one full cycle of the input step pattern
    let ends_check = BTreeMap::from([
        (c("VXB"), vec![(280, c("NTZ"))]),
        (c("VBD"), vec![(280, c("DLZ"))]),
        (c("JLD"), vec![(280, c("BGZ"))]),
        (c("HNH"), vec![(280, c("ZZZ"))]),
        (c("CJJ"), vec![(280, c("BGZ"))]),
        (c("CDL"), vec![(280, c("HBZ"))]),
        (c("RJP"), vec![(280, c("NTZ"))]),
        (c("KLQ"), vec![(280, c("HBZ"))]),
        (c("CCR"), vec![(280, c("ZZZ"))]),
        (c("LCR"), vec![(280, c("RGZ"))]),
        (c("SST"), vec![(280, c("DLZ"))]),
        (c("CPX"), vec![(280, c("RGZ"))]),
    ]);

    assert_eq!(ends.len(), 12);
    assert_eq!(ends, ends_check);

    let states = data
        .start
        .iter()
        .copied()
        .map(|s| (s, map_state(&cycle_state, s)))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        states,
        BTreeMap::from([
            (c("AAA"), (59, c("ZZZ"))),
            (c("NFA"), (43, c("HBZ"))),
            (c("LJA"), (79, c("BGZ"))),
            (c("PLA"), (71, c("RGZ"))),
            (c("KTA"), (53, c("DLZ"))),
            (c("JXA"), (61, c("NTZ"))),
        ])
    );

    let end_states = data
        .end
        .iter()
        .copied()
        .map(|s| (s, map_state(&cycle_state, s)))
        .collect::<BTreeMap<_, _>>();

    // Two notes: they all point into each other, AND the cycle length is the same as for
    // for the basic one.
    assert_eq!(
        end_states,
        BTreeMap::from([
            (c("HBZ"), (43, c("HBZ"))),
            (c("BGZ"), (79, c("BGZ"))),
            (c("RGZ"), (71, c("RGZ"))),
            (c("DLZ"), (53, c("DLZ"))),
            (c("NTZ"), (61, c("NTZ"))),
            (c("ZZZ"), (59, c("ZZZ"))),
        ])
    );

    let mut s = end_states
        .values()
        .map(|(cycle_len, _s)| *cycle_len * steps_per_cycle)
        .collect::<Vec<_>>();
    s.sort();
    assert_eq!(s, vec![12083, 14893, 16579, 17141, 19951, 22199]);
    s.into_iter().fold(1, lcm)
}

fn map_state(cycle_state: &BTreeMap<StateID, StateID>, s: StateID) -> (usize, StateID) {
    StateIter::new(cycle_state, s)
        .enumerate()
        .find(|(_, s)| s.is_end())
        .map(|(i, s)| (i + 1, s))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{c, StateID};
    use std::path::Path;

    #[test]
    fn example1() {
        let c1 = super::part1(Path::new("res/day8/example.txt"));
        assert_eq!(c1, 2);
    }

    #[test]
    fn example1b() {
        let c1 = super::part1(Path::new("res/day8/example2.txt"));
        assert_eq!(c1, 6);
    }

    #[test]
    fn part1() {
        let c1 = super::part1(Path::new("res/day8/input.txt"));
        assert_eq!(c1, 16579);
    }

    #[test]
    fn state_id() {
        assert_eq!("ABC", "ABC".parse::<StateID>().unwrap().as_str());
        assert!(!c("ABC").is_end());
        assert!(!c("ZBC").is_end());
        assert!(c("ABZ").is_end());
    }

    #[test]
    fn part2() {
        let steps = super::part2_special_case(Path::new("res/day8/input.txt"));
        assert_eq!(steps, 12927600769609);
    }
}
