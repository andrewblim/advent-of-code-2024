use std::fs;
use itertools::Itertools;

pub fn problem1() -> i64 {
    let data = fs::read_to_string("data/day01.txt")
        .expect("Error reading file");
    return problem1_str(data)
}

pub fn problem2() -> () {
    println!("hello, world");
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

    let total_diffs: i64 = values1
        .into_iter()
        .zip(values2)
        .fold(0, |acc, (v1, v2)| acc + (v1 - v2).abs());
    return total_diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_test() {
        let input = String::from("\
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ");
        let result = problem1_str(input);
        assert_eq!(result, 11);
    }
}
