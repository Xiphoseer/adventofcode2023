use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

pub enum Task {
    /// Possible
    Task1 {
        has_red: usize,
        has_green: usize,
        has_blue: usize,
    },
    /// Power-Set
    Task2,
}

pub fn run(path: &str, task: Task) -> io::Result<usize> {
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut buf = String::new();

    let mut sum = 0;

    while let Ok(len) = reader.read_line(&mut buf) {
        if len == 0 {
            break;
        }
        let line = buf.trim();
        let line = line.strip_prefix("Game ").unwrap();
        let (num, line) = line.split_once(':').unwrap();
        let id: usize = num.parse().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for part in line.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for pair in part.split(',') {
                let (count, color) = pair.trim().split_once(' ').unwrap();
                let count: usize = count.trim().parse().unwrap();
                let color: Color = color.trim().parse().unwrap();
                match color {
                    Color::Red => red += count,
                    Color::Green => green += count,
                    Color::Blue => blue += count,
                }
            }

            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }

        match task {
            Task::Task1 {
                has_red,
                has_green,
                has_blue,
            } => {
                if max_red <= has_red && max_green <= has_green && max_blue <= has_blue {
                    sum += id;
                }
            }
            Task::Task2 => {
                sum += max_red * max_green * max_blue;
            }
        }

        buf.clear();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn example() {
        let id_sum = super::run(
            "res/day2/example.txt",
            Task::Task1 {
                has_red: 12,
                has_green: 13,
                has_blue: 14,
            },
        )
        .unwrap();
        assert_eq!(id_sum, 8);
    }

    #[test]
    fn step1() {
        let id_sum = super::run(
            "res/day2/input.txt",
            Task::Task1 {
                has_red: 12,
                has_green: 13,
                has_blue: 14,
            },
        )
        .unwrap();
        assert_eq!(id_sum, 2563);
    }

    #[test]
    fn step2() {
        let power_set_sum = super::run("res/day2/input.txt", Task::Task2).unwrap();
        assert_eq!(power_set_sum, 70768);
    }
}
