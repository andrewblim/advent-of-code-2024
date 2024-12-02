use std::fs;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

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

fn problem2_str(data: String) -> i64 {
    unimplemented!()
}

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
    Either,
}

fn line_ok1(line: &str) -> bool {
    let (ok, _) = line
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .tuple_windows()
        .fold_while((true, Direction::Either), |(ok, dir), (x, y)| {
            let diff = (x - y).abs();
            if diff < 1 || diff > 3 {
                Done((false, dir))
            } else {
                match dir {
                    Direction::Increasing => if x >= y { Done((false, dir)) } else { Continue((ok, dir)) },
                    Direction::Decreasing => if y >= x { Done((false, dir)) } else { Continue((ok, dir)) },
                    _ => Continue((ok, if x < y { Direction::Increasing } else { Direction::Decreasing } )),
                }
            }
        })
        .into_inner();
    ok
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
}
