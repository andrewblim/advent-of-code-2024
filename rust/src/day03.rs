use std::fs;
use regex::Regex;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day03.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&data)
        .map(|cap| {
            &cap[1].parse::<u64>().unwrap() * &cap[2].parse::<u64>().unwrap()
        })
        .sum()
}

fn problem2_str(data: String) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let (total, _) = re.captures_iter(&data)
        .fold((0, true), |(total, enabled), cap| {
            match &cap[0] {
                "do()" => (total, true),
                "don't()" => (total, false),
                _ => {
                    let new_total = total + if enabled {
                        &cap[1].parse::<u64>().unwrap() * &cap[2].parse::<u64>().unwrap()
                    } else {
                        0
                    };
                    (new_total, enabled)
                }
            }
        });
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    #[fixture]
    fn input2() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 161);
    }

    #[rstest]
    fn problem2_test(input2: String) {
        assert_eq!(problem2_str(input2), 48);
    }
}
