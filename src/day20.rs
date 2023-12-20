use std::{
    collections::{BTreeMap, VecDeque},
    path::Path,
};

fn is_not_empty(line: &&str) -> bool {
    !line.is_empty()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    /// Flip-Flop
    FlipFlop,
    /// Conjunction
    Nand,
}

pub fn run(path: &Path) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let mut broadcaster = vec![];
    let mut usage = BTreeMap::<&str, usize>::new();
    let logic = text
        .lines()
        .filter(is_not_empty)
        .filter_map(|line| {
            let (key, value) = line.split_once(" -> ").unwrap();
            let dest = value
                .split(", ")
                .map(|n| {
                    let e = usage.entry(n).or_default();
                    (n, std::mem::replace(e, *e + 1))
                })
                .collect::<Vec<_>>();
            if key == "broadcaster" {
                assert!(broadcaster.is_empty(), "broadcaster already set");
                broadcaster = dest;
                None
            } else if let Some(name) = key.strip_prefix('%') {
                Some((name, (Kind::FlipFlop, dest)))
            } else if let Some(name) = key.strip_prefix('&') {
                Some((name, (Kind::Nand, dest)))
            } else {
                panic!()
            }
        })
        .collect::<BTreeMap<_, _>>();
    eprintln!("logic: {logic:?}");
    eprintln!("usage: {usage:?}");

    let mut logic_ids = BTreeMap::<&str, usize>::new();
    let mut i = 0;
    for (name, (kind, _)) in &logic {
        let l_use = usage.get(name).copied().unwrap();
        match kind {
            Kind::FlipFlop => {
                logic_ids.insert(name, i);
                i += 1;
            }
            Kind::Nand => {
                logic_ids.insert(name, i);
                for _ in 0..l_use {
                    i += 1;
                }
            }
        }
    }

    for (name, _cnt) in usage.iter() {
        logic_ids.entry(name).or_insert_with(|| {
            let index = i;
            i += 1;
            index
        });
    }
    eprintln!("ids: {logic_ids:?}");

    let mut state = 0u128;
    let mut index = 0usize;
    //let mut record = BTreeMap::new();
    //let mut timings = vec![];
    let mut sum = (0, 0);
    let mut rx_index = 0;
    while index < 1000 {
        //record.insert(state, index);
        let (low, high, rx) = push_button(&broadcaster, &logic, &logic_ids, &mut state, &usage);
        //eprintln!("{index:>4} low={low:<3}, high={high:<3} 0x{state:032x}");
        //timings.push((low, high));
        if index < 1000 {
            sum.0 += low;
            sum.1 += high;
        }
        index += 1;

        if rx && rx_index == 0 {
            rx_index = index;
        }
    }
    /*if let Some(repeat_index) = record.get(&state) {
        eprintln!("repeat to {repeat_index} at {index}");
    } else {
        eprintln!("Clicked the button {index} times");
    }*/
    eprintln!("{sum:?}");
    eprintln!("{rx_index}");
    //eprintln!("{:?}", timings);
    //eprintln!("{:?}", record);
    sum.0 * sum.1
}

pub type LogicMap<'a> = BTreeMap<&'a str, (Kind, Vec<(&'a str, usize)>)>;

fn push_button(
    broadcaster: &[(&str, usize)],
    logic: &LogicMap,
    logic_ids: &BTreeMap<&str, usize>,
    state: &mut u128,
    usage: &BTreeMap<&str, usize>,
) -> (usize, usize, bool) {
    let mut queue = VecDeque::<Signal>::new();
    // Start out with all broadcast signals in the queue
    for signal in broadcaster.iter().copied().map(|(target, ofs)| Signal {
        src: "broadcaster",
        target,
        ofs,
        kind: SignalKind::Low,
    }) {
        queue.push_back(signal);
    }

    let mut low_count = 1; // button -> broadcaster
    let mut high_count = 0;
    let mut rx = false;
    //eprintln!("<-- click :D -->");
    while let Some(next) = queue.pop_front() {
        match next.kind {
            SignalKind::Low => low_count += 1,
            SignalKind::High => high_count += 1,
        }
        //eprintln!("next: {next:?} 0x{state:032x}");
        let src = next.target;
        if let Some((mod_kind, dest)) = logic.get(src) {
            match (*mod_kind, next.kind) {
                (Kind::FlipFlop, SignalKind::Low) => {
                    let index = *logic_ids.get(src).unwrap();
                    let bit = 1u128 << index;
                    *state ^= bit; // toggle bit
                    let kind = if (*state & bit) > 0 {
                        SignalKind::High
                    } else {
                        SignalKind::Low
                    };
                    for (target, ofs) in dest.iter().copied() {
                        queue.push_back(Signal {
                            kind,
                            src,
                            target,
                            ofs,
                        });
                    }
                }
                (Kind::FlipFlop, SignalKind::High) => { /* ignore */ }
                (Kind::Nand, s) => {
                    let index = *logic_ids.get(src).unwrap();
                    let bit = 1u128 << (index + next.ofs);

                    let len = *usage.get(src).unwrap();
                    let mask = ((1u128 << len) - 1) << index;

                    match s {
                        SignalKind::Low => {
                            *state &= !bit; // unset bit
                        }
                        SignalKind::High => {
                            *state |= bit; // set bit
                        }
                    }

                    let kind = if (*state & mask) == mask {
                        SignalKind::Low
                    } else {
                        SignalKind::High
                    };

                    // all bits high
                    for (target, ofs) in dest.iter().copied() {
                        let signal = Signal {
                            kind,
                            src,
                            target,
                            ofs,
                        };
                        queue.push_back(signal);
                    }
                }
            }
        } else if src == "rx" && next.kind == SignalKind::Low {
            rx = true;
        }
    }
    (low_count, high_count, rx)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignalKind {
    Low,
    High,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Signal<'a> {
    src: &'a str,
    kind: SignalKind,
    target: &'a str,
    ofs: usize,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn example1() {
        assert_eq!(run(Path::new("res/day20/example1.txt")), 32000000);
    }

    #[test]
    fn example2() {
        assert_eq!(run(Path::new("res/day20/example2.txt")), 11687500);
    }

    #[test]
    fn part1() {
        assert_eq!(run(Path::new("res/day20/input.txt")), 730797576);
    }

    #[test]
    fn part2() {
        run(Path::new("res/day20/input.txt"));
    }
}
