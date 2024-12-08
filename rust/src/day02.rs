use std::fs;
use itertools::Itertools;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day02.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    data
        .trim()
        .split("\n")
        .filter(|line| line_ok1(line))
        .count()
}

fn problem2_str(data: String) -> usize {
    data
        .trim()
        .split("\n")
        .filter(|line| line_ok2(line))
        .count()
}

fn line_ok1(line: &str) -> bool {
    let diffs = get_diffs(line);
    diffs_ok(&diffs)
}

fn line_ok2(line: &str) -> bool {
    let diffs = get_diffs(line);
    diffs_ok(&diffs) || 
        diffs_ok(&diffs[1..]) ||
        diffs_ok(&diffs[..(diffs.len() - 1)]) ||
        (0..(diffs.len() - 1)).any(|i| {
            let diffs_i = [&diffs[..i], &[diffs[i] + diffs[i + 1]], &diffs[(i + 2)..]].concat();
            diffs_ok(&diffs_i)
        })
}

fn get_diffs(line: &str) -> Vec<i64> {
    line
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect()
}

fn diffs_ok(diffs: &[i64]) -> bool {
    match diffs[..] {
        [] => true,
        [d0] if d0.abs() < 1 || d0.abs() > 3 => false,
        [d0, ..] => diffs.iter().all(|d| d.abs() >= 1 && d.abs() <= 3 && d.signum() == d0.signum()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 2);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 4);
    }
}
