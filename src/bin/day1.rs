use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use regex::Regex;

#[derive(argh::FromArgs)]
/// Day 1 Challenge
struct Options {
    #[argh(positional)]
    /// input file path
    path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let opts: Options = argh::from_env();
    let mut reader = BufReader::new(File::open(opts.path)?);

    let last = ".*(one|two|three|four|five|six|seven|eight|nine|[0-9])";
    let basic = &last[2..];
    let regex_two = Regex::new(last).unwrap();
    let regex_one = Regex::new(basic).unwrap();

    let mut acc = 0;
    let mut buf = String::new();
    while let Ok(bytes) = reader.read_line(&mut buf) {
        if bytes == 0 {
            break;
        }

        let line = buf.trim();
        let first_str = regex_one.find(line).unwrap().as_str();
        let last_str = regex_two.captures(line).unwrap().get(1).unwrap().as_str();

        let first = parse(first_str).unwrap();
        let last = parse(last_str).unwrap();

        let num = first * 10 + last;

        println!("{line:?} {first_str} {last_str} {first} {last} {num}");

        acc += num;

        buf.clear();
    }

    eprintln!("Answer: {}", acc); // Wrong: 54953, Correct Answer: 54925

    Ok(())
}

fn parse(k: &str) -> Option<usize> {
    match k {
        "0" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn _digit_iter(buf: &str, zero: usize) -> impl Iterator<Item = usize> + DoubleEndedIterator + '_ {
    buf.chars()
        .filter(|c| c.is_ascii_digit())
        .map(move |x| x as usize - zero)
}
