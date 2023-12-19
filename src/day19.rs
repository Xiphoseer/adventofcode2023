use std::{collections::BTreeMap, ops::Range, path::Path, str::Split};

#[derive(Debug)]
pub struct Item {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Item {
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_m(&self) -> usize {
        self.m
    }
    pub fn get_a(&self) -> usize {
        self.a
    }
    pub fn get_s(&self) -> usize {
        self.s
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn of_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!(),
        }
    }

    fn apply(&self, item: &Item) -> usize {
        match self {
            Field::X => item.get_x(),
            Field::M => item.get_m(),
            Field::A => item.get_a(),
            Field::S => item.get_s(),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Gt,
    Lt,
}

impl Op {
    fn of_char(c: char) -> Self {
        match c {
            '<' => Self::Lt,
            '>' => Self::Gt,
            _ => panic!(),
        }
    }

    fn apply(&self, a: usize, b: usize) -> bool {
        match self {
            Op::Gt => greater_than(a, b),
            Op::Lt => less_than(a, b),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub field: Field,
    pub case_true: usize,
    pub case_false: usize,
    pub op: Op,
    pub operand: usize,
}

impl State {
    fn apply(&self, item: &Item) -> usize {
        if self.op.apply(self.field.apply(item), self.operand) {
            self.case_true
        } else {
            self.case_false
        }
    }
}

fn less_than(a: usize, b: usize) -> bool {
    a < b
}

fn greater_than(a: usize, b: usize) -> bool {
    a > b
}

#[derive(Debug, Clone)]
struct ValidRanges {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl ValidRanges {
    pub fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    pub fn count(&self, transitions: &BTreeMap<usize, State>, state: usize) -> usize {
        if state == REJECT {
            0
        } else if state == ACCEPT {
            self.x.len() * self.m.len() * self.a.len() * self.s.len()
        } else {
            let t = transitions.get(&state).unwrap();
            let Range { start, end } = match t.field {
                Field::X => &self.x,
                Field::M => &self.m,
                Field::A => &self.a,
                Field::S => &self.s,
            };
            match t.op {
                Op::Gt => {
                    let mid = t.operand + 1; // first match for x > operand
                    if mid >= *end {
                        self.count(transitions, t.case_false)
                    } else if *start > t.operand {
                        self.count(transitions, t.case_true)
                    } else {
                        let a = self
                            .with(t.field, *start..mid)
                            .count(transitions, t.case_false);
                        let b = self
                            .with(t.field, mid..*end)
                            .count(transitions, t.case_true);
                        a + b
                    }
                }
                Op::Lt => {
                    if t.operand <= *start {
                        self.count(transitions, t.case_false)
                    } else if *end <= t.operand {
                        self.count(transitions, t.case_true)
                    } else {
                        let a = self
                            .with(t.field, *start..t.operand)
                            .count(transitions, t.case_true);
                        let b = self
                            .with(t.field, t.operand..*end)
                            .count(transitions, t.case_false);
                        a + b
                    }
                }
            }
        }
    }

    fn with(&self, field: Field, operand: Range<usize>) -> Self {
        let mut c = self.clone();
        *match field {
            Field::X => &mut c.x,
            Field::M => &mut c.m,
            Field::A => &mut c.a,
            Field::S => &mut c.s,
        } = operand;
        c
    }
}

pub fn run(path: &Path) -> (usize, usize) {
    let text = std::fs::read_to_string(path).unwrap();
    let (_state_strs, item_strs) = text.split_once("\n\n").unwrap();
    let items = parse_items(item_strs);
    let (in_state, transitions) = parse_states(_state_strs);
    eprintln!("initial state: {:#?}", in_state);
    eprintln!("#transitions: {}", transitions.len());

    // Part 1
    let mut sum = 0;
    for item in items {
        eprintln!("{item:?}");
        if apply(&transitions, in_state, &item) {
            sum += item.sum();
        }
    }

    // Part 2
    let count = ValidRanges::new().count(&transitions, in_state);

    (sum, count)
}

fn apply(transitions: &BTreeMap<usize, State>, mut state: usize, item: &Item) -> bool {
    while let Some(t) = transitions.get(&state) {
        let new_state = t.apply(item);
        eprintln!("{t:?} => {new_state}");
        state = new_state;
    }
    match state {
        ACCEPT => true,
        REJECT => false,
        _ => panic!("{state}"),
    }
}

fn parse_states(_state_strs: &str) -> (usize, BTreeMap<usize, State>) {
    const SUBSTATE_BITS: usize = 8;
    let state_lines = _state_strs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let def = rest.strip_suffix('}').unwrap();
            (name, def)
        })
        .collect::<BTreeMap<_, _>>();
    let state_ids = state_lines
        .keys()
        .copied()
        .enumerate()
        .map(|(i, name)| (name, i << SUBSTATE_BITS))
        .collect::<BTreeMap<_, _>>();
    let in_state = *state_ids.get("in").unwrap();
    let transitions = state_lines
        .into_iter()
        .enumerate()
        .flat_map(|(i, (name, def))| {
            eprintln!("{name}: {i}");
            let state_ids = &state_ids;
            let mut iter = def.split(',');
            let last = iter.next_back().unwrap();
            let last = get_state_id(state_ids, last);
            let mut iter = iter.enumerate().peekable();
            std::iter::from_fn(move || {
                if let Some((j, def)) = iter.next() {
                    let state_id = (i << SUBSTATE_BITS) + j;
                    let (cond, next) = def.split_once(':').unwrap();
                    let case_true = get_state_id(state_ids, next);
                    let case_false = if iter.peek().is_some() {
                        state_id + 1
                    } else {
                        last
                    };
                    let mut cond_chars = cond.chars();
                    let field = Field::of_char(cond_chars.next().unwrap());
                    let op = Op::of_char(cond_chars.next().unwrap());
                    let operand = cond_chars.as_str().parse().unwrap();
                    Some((
                        state_id,
                        State {
                            field,
                            case_true,
                            case_false,
                            op,
                            operand,
                        },
                    ))
                } else {
                    None
                }
            })
        })
        .collect();
    (in_state, transitions)
}

const ACCEPT: usize = usize::MAX;
const REJECT: usize = usize::MAX - 1;

fn get_state_id(state_ids: &BTreeMap<&str, usize>, last: &str) -> usize {
    match last {
        "A" => ACCEPT,
        "R" => REJECT,
        _ => *state_ids.get(last).unwrap(),
    }
}

fn parse_items(item_strs: &str) -> Vec<Item> {
    item_strs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.strip_prefix('{').unwrap();
            let line = line.strip_suffix('}').unwrap();
            let mut iter = line.split(',');
            let x = scan_line("x=", &mut iter);
            let m = scan_line("m=", &mut iter);
            let a = scan_line("a=", &mut iter);
            let s = scan_line("s=", &mut iter);
            Item { x, m, a, s }
        })
        .collect()
}

fn scan_line(prefix: &'static str, iter: &mut Split<'_, char>) -> usize {
    iter.next()
        .unwrap()
        .strip_prefix(prefix)
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn example() {
        let (part1, part2) = run(Path::new("res/day19/example.txt"));
        assert_eq!(part1, 19114);
        assert_eq!(part2, 167409079868000);
    }

    #[test]
    fn input() {
        let (part1, part2) = run(Path::new("res/day19/input.txt"));
        assert_eq!(part1, 432434);
        assert_eq!(part2, 132557544578569);
    }
}
