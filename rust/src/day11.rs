use std::fs;

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
    let mut stones: Vec<usize> = data.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    
    for _ in 0..25 {
        stones = transform_stones(&stones);
    }
    stones.len()
}

fn problem2_str(data: String) -> usize {
    unimplemented!();
}

fn transform_stones(stones: &[usize]) -> Vec<usize> {
    let mut new_stones = Vec::new();
    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let n_digits = (stone as f64).log10().floor() as u32 + 1;
            if n_digits % 2 == 0 {
                let divisor = 10_usize.checked_pow(n_digits / 2).unwrap();
                new_stones.push(stone / divisor);
                new_stones.push(stone % divisor);
            } else {
                new_stones.push(stone * 2024);
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
