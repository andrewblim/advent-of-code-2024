use std::{collections::HashMap, fs};
use itertools::Itertools;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day11.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    let mut stones = data.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .counts();
    
    for _ in 0..25 {
        stones = transform_stones(&stones);
    }
    stones.values().sum()
}

fn problem2_str(data: String) -> usize {
    let mut stones = data.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .counts();
    
    for _ in 0..75 {
        stones = transform_stones(&stones);
    }
    stones.values().sum()
}

fn transform_stones(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    for (&stone, &count) in stones {
        if stone == 0 {
            new_stones.entry(1).and_modify(|e| *e += count).or_insert(count);
        } else {
            let n_digits = (stone as f64).log10().floor() as u32 + 1;
            if n_digits % 2 == 0 {
                let divisor = 10_usize.checked_pow(n_digits / 2).unwrap();
                new_stones.entry(stone / divisor).and_modify(|e| *e += count).or_insert(count);
                new_stones.entry(stone % divisor).and_modify(|e| *e += count).or_insert(count);
            } else {
                new_stones.entry(stone * 2024).and_modify(|e| *e += count).or_insert(count);
            }
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("125 17")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 55312);
    }
}
