use std::collections::{btree_map::Entry, BTreeMap};

pub enum GearState {
    One(usize),
    Two(usize),
    ThreeOrMore,
}

pub enum Task {
    Part1,
    Part2 {
        maybe_gears: BTreeMap<usize, GearState>,
    },
}

pub fn read_schematic(path: &str, mut task: Task) -> usize {
    let schematic = std::fs::read_to_string(path).unwrap();
    let stride = schematic.split_once('\n').unwrap().0.len() + 1;
    let height = schematic.len() / stride;

    assert_eq!(stride * height, schematic.len());

    let numbers = regex::Regex::new("[0-9]+").unwrap();

    let mut sum = 0;
    for (y, line) in schematic.lines().enumerate() {
        if line.is_empty() {
            break;
        }

        for _match in numbers.find_iter(line) {
            let x = _match.start();
            let text = _match.as_str();
            let len = text.len();
            let value: usize = text.parse().unwrap();
            let mut is_part = false;
            let mut gear_pos = vec![];

            if y > 0 {
                // Check above
                if x > 0 {
                    // top-left
                    let pos = (y - 1) * stride + (x - 1);
                    let c = schematic.as_bytes()[pos];
                    is_part |= is_symbol(c as char);
                    if is_gear(c) {
                        gear_pos.push(pos);
                    }
                }
                for i in 0..=len {
                    let pos = (y - 1) * stride + (x + i);
                    let c = schematic.as_bytes()[pos];
                    is_part |= is_symbol(c as char);
                    if is_gear(c) {
                        gear_pos.push(pos);
                    }
                }
            }

            // Check left
            if x > 0 {
                // left
                let pos = y * stride + (x - 1);
                let c = schematic.as_bytes()[pos];
                is_part |= is_symbol(c as char);
                if is_gear(c) {
                    gear_pos.push(pos);
                }
            }

            // Check right
            let pos = y * stride + (x + len);
            let c = schematic.as_bytes()[pos];
            is_part |= is_symbol(c as char);
            if is_gear(c) {
                gear_pos.push(pos);
            }

            if y + 1 < height {
                // Check below
                if x > 0 {
                    // bottom-left
                    let pos = (y + 1) * stride + (x - 1);
                    let c = schematic.as_bytes()[pos];
                    is_part |= is_symbol(c as char);
                    if is_gear(c) {
                        gear_pos.push(pos);
                    }
                }
                for i in 0..=len {
                    let pos = (y + 1) * stride + (x + i);
                    let c = schematic.as_bytes()[pos];
                    is_part |= is_symbol(c as char);
                    if is_gear(c) {
                        gear_pos.push(pos);
                    }
                }
            }

            if is_part {
                match &mut task {
                    Task::Part1 => {
                        sum += value; // part id
                    }
                    Task::Part2 { maybe_gears } => {
                        for pos in gear_pos {
                            match maybe_gears.entry(pos) {
                                Entry::Vacant(v) => {
                                    v.insert(GearState::One(value));
                                }
                                Entry::Occupied(mut o) => {
                                    o.insert(match o.get() {
                                        GearState::One(value1) => GearState::Two(value * value1),
                                        GearState::Two(_) | GearState::ThreeOrMore => {
                                            GearState::ThreeOrMore
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match task {
        Task::Part1 => sum,
        Task::Part2 { maybe_gears } => maybe_gears
            .into_iter()
            .filter_map(|(_, f)| match f {
                GearState::One(_) => None,
                GearState::Two(ratio) => Some(ratio),
                GearState::ThreeOrMore => None,
            })
            .sum(),
    }
}

fn is_symbol(c: char) -> bool {
    !matches!(c, '.' | '\r' | '\n' | '0'..='9')
}

fn is_gear(c: u8) -> bool {
    c == b'*'
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::day3::Task;

    #[test]
    fn example() {
        let sum = super::read_schematic("res/day3/example.txt", Task::Part1);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn part1() {
        let sum = super::read_schematic("res/day3/input.txt", Task::Part1);
        assert_eq!(sum, 540025);
    }

    #[test]
    fn example2() {
        let sum = super::read_schematic(
            "res/day3/example2.txt",
            Task::Part2 {
                maybe_gears: BTreeMap::new(),
            },
        );
        assert_eq!(sum, 467835);
    }

    #[test]
    fn part2() {
        let sum = super::read_schematic(
            "res/day3/input.txt",
            Task::Part2 {
                maybe_gears: BTreeMap::new(),
            },
        );
        assert_eq!(sum, 84584891);
    }
}
