use std::fs;
use itertools::Itertools;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day01.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> i64 {
    let mut values1 = Vec::<i64>::new();
    let mut values2 = Vec::<i64>::new();

    for line in data.trim().split("\n") {
        let (v1, v2) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        values1.push(v1);
        values2.push(v2);
    }

    values1.sort();
    values2.sort();

    values1
        .into_iter()
        .zip(values2)
        .fold(0, |acc, (v1, v2)| acc + (v1 - v2).abs())
}

fn problem2_str(data: String) -> i64 {
    let mut values1 = Vec::<i64>::new();
    let mut values2 = Vec::<i64>::new();

    for line in data.trim().split("\n") {
        let (v1, v2) = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        values1.push(v1);
        values2.push(v2);
    }

    let counts2 = values2.into_iter().counts();
    values1
        .into_iter()
        .fold(0, |acc, v1| {
            acc + match counts2.get(&v1) {
                Some(x) => v1 * (*x as i64),
                None => 0
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
3   4
4   3
2   5
1   3
3   9
3   3
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 11);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 31);
    }
}
