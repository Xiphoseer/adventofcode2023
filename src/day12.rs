pub fn parse(s: &str) -> (&str, Vec<usize>) {
    let (pattern, rest) = s.split_once(' ').unwrap();
    let v = rest
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();
    (pattern, v)
}

pub mod part1 {
    use std::path::Path;

    pub fn variants<I: Iterator<Item = char> + Clone, L: Iterator<Item = usize> + Clone>(
        mut chars: I,
        lengths: L,
    ) -> usize {
        match chars.next() {
            Some('?') => {
                let rest = chars.clone();
                consume_broken(chars, lengths.clone()) + variants(rest, lengths)
            }
            Some('#') => consume_broken(chars, lengths),
            Some('.') => variants(chars, lengths),
            Some(_) => panic!(),
            None => check_no_more_lengths(lengths),
        }
    }

    fn consume_broken<I: Iterator<Item = char> + Clone, L: Iterator<Item = usize> + Clone>(
        mut chars: I,
        mut lengths: L,
    ) -> usize {
        match lengths.next() {
            Some(len) => {
                for _i in 1..len {
                    let pat = chars.next();
                    // len - 1
                    match pat {
                        Some('.') | None => return 0, // missing # or ?
                        Some('#' | '?') => {}
                        Some(_) => panic!(),
                    }
                }
                let pat = chars.next();
                match pat {
                    Some('.' | '?') => variants(chars, lengths),
                    Some('#') => 0,
                    Some(_) => panic!(),
                    None => check_no_more_lengths(lengths),
                }
            }
            None => 0,
        }
    }

    fn check_no_more_lengths<L: Iterator<Item = usize>>(mut lengths: L) -> usize {
        if lengths.next().is_none() {
            1
        } else {
            0
        }
    }

    pub fn run(path: &Path, mul: usize) -> usize {
        let text = std::fs::read_to_string(path).unwrap();
        text.lines()
            .map(super::parse)
            .map(|(pattern, lengths)| {
                let chars = pattern.chars().cycle().take(pattern.len() * mul);
                let lengths = lengths.iter().copied().cycle().take(lengths.len() * mul);
                variants(chars, lengths)
            })
            .sum()
    }
}

pub mod part2 {
    use std::{collections::BTreeMap, path::Path};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum State {
        // Free to start a new broken sequence
        SeqStart,
        // Seen a '#' already, more to come
        Seq,
        // Immediately after a '#'
        SeqEnd,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Pattern {
        // '#'
        Broken,
        // '.'
        Working,
        // '?'
        Any,
    }

    pub fn variants(pattern: &str, lengths: &[usize], mul: usize) -> usize {
        let states = make_states(lengths, mul);
        let final_state = nfa(&states, pattern, 5);
        // Check the end-state (past the end of our state vector)
        let post_state_id = states.len();
        let post_count = final_state.get(&post_state_id).copied().unwrap_or(0);
        // Check the last end state (we don't need trailing '.' and this makes the loop easier)
        let final_state_id = post_state_id - 1;
        let final_count = final_state.get(&final_state_id).copied().unwrap_or(0);
        // Add those together
        post_count + final_count
    }

    pub fn nfa(states: &[State], input: &str, mul: usize) -> BTreeMap<usize, usize> {
        let start = BTreeMap::from([(0, 1)]);
        let char_seq_count = (input.len() + 1) * mul - 1; // len + '?', mul times, but no trailing '?'
        let chars = input.chars().chain(Some('?')).cycle().take(char_seq_count);
        let mut state = start;
        for c in chars {
            state = nfa_step(states, &state, c);
        }
        state
    }

    pub fn nfa_step(
        states: &[State],
        input: &BTreeMap<usize, usize>,
        c: char,
    ) -> BTreeMap<usize, usize> {
        let mut next = BTreeMap::new();
        let pat = match c {
            '#' => Pattern::Broken,
            '.' => Pattern::Working,
            '?' => Pattern::Any,
            _ => panic!("{c}"),
        };
        for (&state, &count) in input {
            let kind = states.get(state).copied();
            match (kind, pat) {
                // We have a true choice
                (Some(State::SeqStart), Pattern::Any) => {
                    *next.entry(state + 1).or_default() += count;
                    *next.entry(state).or_default() += count;
                }
                // The current state forces us to pick '#' or '.'
                (Some(State::SeqStart), Pattern::Broken)
                | (Some(State::Seq), Pattern::Any | Pattern::Broken)
                | (Some(State::SeqEnd), Pattern::Any | Pattern::Working) => {
                    *next.entry(state + 1).or_default() += count;
                }
                // The current state forces us to remain in the current state (no '#')
                (Some(State::SeqStart), Pattern::Working)
                | (None, Pattern::Any | Pattern::Working) => {
                    *next.entry(state).or_default() += count;
                }
                // The current state forces us into the failure state
                (Some(State::Seq), Pattern::Working)
                | (Some(State::SeqEnd), Pattern::Broken)
                | (None, Pattern::Broken) => {
                    // will not generate new states
                }
            }
        }
        next
    }

    pub(crate) fn make_states(lengths: &[usize], mul: usize) -> Vec<State> {
        let lengths = lengths.iter().copied().cycle().take(lengths.len() * mul);
        let mut states = vec![];
        for len in lengths {
            states.push(State::SeqStart);
            for _i in 1..len {
                states.push(State::Seq);
            }
            states.push(State::SeqEnd);
        }
        states
    }

    pub fn run(path: &Path, mul: usize) -> usize {
        let text = std::fs::read_to_string(path).unwrap();
        text.lines()
            .map(super::parse)
            .map(|(pattern, lengths)| variants(pattern, &lengths, mul))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use super::{
        part1::{run, variants},
        part2,
    };

    #[test]
    fn parse() {
        let (pattern, lengths) = super::parse("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(pattern, "?#?#?#?#?#?#?#?");
        assert_eq!(lengths, &[1, 3, 1, 6]);
        assert_eq!(variants(pattern.chars(), lengths.iter().copied()), 1);

        let (pattern, lengths) = super::parse("?###???????? 3,2,1");
        assert_eq!(pattern, "?###????????");
        assert_eq!(lengths, &[3, 2, 1]);
        assert_eq!(variants(pattern.chars(), lengths.iter().copied()), 10);
    }

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day12/example.txt"), 1), 21);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day12/input.txt"), 1), 6827);
    }

    #[test]
    fn example2() {
        let states = part2::make_states(&[1, 1, 3], 5);
        use part2::State::*;
        assert_eq!(
            &states,
            &[
                SeqStart, SeqEnd, SeqStart, SeqEnd, SeqStart, Seq, Seq, SeqEnd, //
                SeqStart, SeqEnd, SeqStart, SeqEnd, SeqStart, Seq, Seq, SeqEnd, //
                SeqStart, SeqEnd, SeqStart, SeqEnd, SeqStart, Seq, Seq, SeqEnd, //
                SeqStart, SeqEnd, SeqStart, SeqEnd, SeqStart, Seq, Seq, SeqEnd, //
                SeqStart, SeqEnd, SeqStart, SeqEnd, SeqStart, Seq, Seq, SeqEnd, //
            ]
        );

        let start = BTreeMap::from([(0usize, 1usize)]);
        assert_eq!(
            part2::nfa_step(&states, &start, '#'),
            BTreeMap::from([(1, 1)])
        );
        assert_eq!(
            part2::nfa_step(&states, &start, '.'),
            BTreeMap::from([(0, 1)])
        );

        // 1,1,3

        let s1 = part2::nfa_step(&states, &start, '?');
        // 0 turns to 0 or 1
        assert_eq!(s1, BTreeMap::from([(0, 1), (1, 1)]));

        let s2 = part2::nfa_step(&states, &s1, '?');
        // 0 turns to 0 or 1, 1 turns to 2
        assert_eq!(s2, BTreeMap::from([(0, 1), (1, 1), (2, 1)]));

        let s3 = part2::nfa_step(&states, &s2, '?');
        // 0 turns to 0 or 1, 1 turns to 2, 2 turns to 2 or 3
        assert_eq!(s3, BTreeMap::from([(0, 1), (1, 1), (2, 2), (3, 1)]));

        let s4 = part2::nfa_step(&states, &s3, '.');
        // 0 turns to 0, 1 turns to 2, 2 turns to 2, 3 turns to 4
        assert_eq!(s4, BTreeMap::from([(0, 1), (2, 3), (4, 1)]));

        let s5 = part2::nfa_step(&states, &s4, '#');
        // 0 turns to 1, 2 turns to 3, 4 turns to 5
        assert_eq!(s5, BTreeMap::from([(1, 1), (3, 3), (5, 1)]));

        let s6 = part2::nfa_step(&states, &s5, '#');
        // 5 turns to 6
        assert_eq!(s6, BTreeMap::from([(6, 1)]));

        let s7 = part2::nfa_step(&states, &s6, '#');
        // 6 turns to 7
        assert_eq!(s7, BTreeMap::from([(7, 1)]));

        assert_eq!(states.len(), 40);
        let final_state = part2::nfa(&states, "???.###", 5);
        assert_eq!(final_state, BTreeMap::from([(39, 1)]));
        assert_eq!(final_state[&(states.len() - 1)], 1);
    }

    #[test]
    fn part2() {
        assert_eq!(
            part2::run(Path::new("res/day12/input.txt"), 5),
            1537505634471
        );
    }
}
