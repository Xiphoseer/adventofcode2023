use std::path::Path;

pub fn run(path: &Path) {
    let _text = std::fs::read_to_string(path).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn example1() {
        run(Path::new("res/dayX/example.txt"));
    }
    
    #[test]
    fn part1() {
        run(Path::new("res/dayX/input.txt"));
    }

    #[test]
    fn example2() {
        run(Path::new("res/dayX/example.txt"));
    }

    #[test]
    fn part2() {
        run(Path::new("res/dayX/input.txt"));
    }
}