use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub enum Part {
    Part1,
    Part2 {
        /// Map from ID to count
        cards: BTreeMap<usize, usize>,
    },
}

pub struct Card {
    pub id: usize,
    winning: BTreeSet<usize>,
    you_have: Vec<usize>,
}

impl Card {
    fn check(&self) -> usize {
        let mut points = 0;
        for num in &self.you_have {
            if self.winning.contains(num) {
                if points == 0 {
                    points = 1;
                } else {
                    points <<= 1;
                }
            }
        }
        points
    }

    fn check_part2(&self) -> usize {
        let mut wins = 0;
        for num in &self.you_have {
            if self.winning.contains(num) {
                wins += 1;
            }
        }
        wins
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.strip_prefix("Card ").unwrap();
        let (id_str, line) = line.split_once(':').unwrap();
        let id: usize = id_str.trim().parse().unwrap();
        let (winning, you_have) = line.split_once('|').unwrap();
        let winning = parse_numbers(winning).collect::<BTreeSet<usize>>();
        let you_have = parse_numbers(you_have).collect::<Vec<usize>>();
        Ok(Self {
            id,
            winning,
            you_have,
        })
    }
}

fn parse_numbers(winning: &str) -> impl Iterator<Item = usize> + '_ {
    winning
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
}

pub fn run(path: &str, mut part: Part) -> usize {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut sum = 0;
    let mut debug = vec![];
    let mut buf = String::new();
    while let Ok(len) = reader.read_line(&mut buf) {
        if len == 0 {
            break;
        }

        let card: Card = buf.trim().parse().unwrap();
        match &mut part {
            Part::Part1 => {
                sum += card.check();
            }
            Part::Part2 { cards } => {
                let copies = cards.get(&card.id).copied().unwrap_or(0) + 1;
                debug.push(copies);
                sum += copies;
                let to_get = card.check_part2();
                for offset in 1..=to_get {
                    *cards.entry(card.id + offset).or_default() += copies;
                }
            }
        }
        buf.clear();
    }
    /*if let Part::Part2 { cards } = &mut part {
        panic!("{debug:?} {cards:?} {sum:?}");
    }*/
    sum
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::Part;

    #[test]
    fn example() {
        let sum = super::run("res/day4/example.txt", Part::Part1);
        assert_eq!(sum, 13);
    }

    #[test]
    fn step1() {
        let sum = super::run("res/day4/input.txt", Part::Part1);
        assert_eq!(sum, 24706);
    }

    #[test]
    fn example2() {
        let sum = super::run(
            "res/day4/example2.txt",
            Part::Part2 {
                cards: BTreeMap::new(),
            },
        );
        assert_eq!(sum, 30);
    }

    #[test]
    fn step2() {
        let sum = super::run(
            "res/day4/input.txt",
            Part::Part2 {
                cards: BTreeMap::new(),
            },
        );
        assert_eq!(sum, 13114317);
    }
}
