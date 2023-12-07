use std::{cmp::Ordering, collections::BTreeMap, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Joker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Jockey,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn part2(&self) -> Self {
        if *self == Self::Jockey {
            Self::Joker
        } else {
            *self
        }
    }

    pub fn from_char(c: char) -> Card {
        match c {
            '2' => Self::C2,
            '3' => Self::C3,
            '4' => Self::C4,
            '5' => Self::C5,
            '6' => Self::C6,
            '7' => Self::C7,
            '8' => Self::C8,
            '9' => Self::C9,
            'T' => Self::T,
            'J' => Self::Jockey,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("{c}"),
        }
    }
}

///
///
/// ```
/// use adventofcode2023::day7::{Hand, Card};
/// use std::str::FromStr;
///
/// assert_eq!("AAAAA".parse(), Ok(Hand([Card::Ace,Card::Ace,Card::Ace,Card::Ace,Card::Ace])));
///
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand(pub [Card; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => self.0.cmp(&other.0),
            ord => ord,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self([
            Card::from_char(chars.next().ok_or(())?),
            Card::from_char(chars.next().ok_or(())?),
            Card::from_char(chars.next().ok_or(())?),
            Card::from_char(chars.next().ok_or(())?),
            Card::from_char(chars.next().ok_or(())?),
        ]))
    }
}

impl Hand {
    pub fn new(s: &str) -> Self {
        s.parse().unwrap()
    }

    pub fn new_part2(s: &str) -> Self {
        let mut hand = Self::new(s);
        for card in &mut hand.0 {
            if *card == Card::Jockey {
                *card = Card::Joker;
            }
        }
        hand
    }

    fn hist(&self) -> (BTreeMap<Card, usize>, usize) {
        let mut map = BTreeMap::new();
        let mut jokers = 0;
        for card in self.0 {
            if card == Card::Joker {
                jokers += 1;
            } else {
                *map.entry(card).or_default() += 1;
            }
        }
        (map, jokers)
    }

    pub fn kind(&self) -> Kind {
        let (hist, _jokers) = self.hist();
        let mut hist: Vec<_> = hist.iter().map(|(card, count)| (*card, *count)).collect();
        hist.sort_by_key(|(_, count)| *count);
        match hist.len() {
            0 => {
                /* all jokers */
                Kind::FiveOfA
            }
            1 => Kind::FiveOfA,
            2 => {
                // Four of a kind or Full House
                let (_, count) = hist.last().unwrap();
                if *count + _jokers == 4 {
                    Kind::FourOfA
                } else {
                    //assert_eq!(*count, 3);
                    Kind::FullHouse
                }
            }
            3 => {
                // Two Pair or Three of a kind
                let (_, count) = hist.last().unwrap();
                if *count + _jokers == 3 {
                    Kind::ThreeOfA
                } else {
                    //assert_eq!(*count, 2);
                    Kind::TwoPair
                }
            }
            4 => Kind::OnePair,
            5 => Kind::HighCard,

            len => panic!("{len}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfA,
    FullHouse,
    FourOfA,
    FiveOfA,
}

pub fn run(path: &Path, parser: impl Fn(&str) -> Hand) -> usize {
    let text = std::fs::read_to_string(path).unwrap();
    let mut bids: Vec<_> = text
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(a, b)| (parser(a), b.parse::<usize>().unwrap()))
        .collect();
    bids.sort_by_key(|(card, _bid)| *card);
    bids.into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

pub fn part1(path: &Path) -> usize {
    run(path, Hand::new)
}

pub fn part2(path: &Path) -> usize {
    run(path, Hand::new_part2)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Hand, Kind};

    #[test]
    fn kinds() {
        assert_eq!(Hand::new("AAAAA").kind(), Kind::FiveOfA);
        assert_eq!(Hand::new("AA8AA").kind(), Kind::FourOfA);
        assert_eq!(Hand::new("23332").kind(), Kind::FullHouse);
        assert_eq!(Hand::new("TTT98").kind(), Kind::ThreeOfA);
        assert_eq!(Hand::new("23432").kind(), Kind::TwoPair);
        assert_eq!(Hand::new("A23A4").kind(), Kind::OnePair);
        assert_eq!(Hand::new("23456").kind(), Kind::HighCard);

        assert!(Kind::FiveOfA > Kind::ThreeOfA);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(Path::new("res/day7/example.txt")), 6440);
        assert_eq!(super::part1(Path::new("res/day7/input.txt")), 251545216);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(Path::new("res/day7/example.txt")), 5905);
        assert_eq!(super::part2(Path::new("res/day7/input.txt")), 250384185);
    }
}
