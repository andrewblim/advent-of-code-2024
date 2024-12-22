use std::fs;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day07.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> u64 {
    let eqns = data
        .trim()
        .split("\n")
        .map(|line| {
            let (result_str, terms_str) = line.split_once(':').unwrap();
            let terms: Vec<u64> = terms_str
                .trim()
                .split_whitespace()
                .map(|term| term.parse::<u64>().unwrap())
                .collect();
            (result_str.parse::<u64>().unwrap(), terms)
        });
    
    eqns
        .filter(|(result, terms) | valid1(*result, terms))
        .map(|(result, _)| result)
        .sum()
}

fn problem2_str(data: String) -> u64 {
    let eqns = data
        .trim()
        .split("\n")
        .map(|line| {
            let (result_str, terms_str) = line.split_once(':').unwrap();
            let terms: Vec<u64> = terms_str
                .trim()
                .split_whitespace()
                .map(|term| term.parse::<u64>().unwrap())
                .collect();
            (result_str.parse::<u64>().unwrap(), terms)
        });
    
    eqns
        .filter(|(result, terms) | valid2(*result, terms))
        .map(|(result, _)| result)
        .sum()
}

fn valid1(target: u64, terms: &[u64]) -> bool {
    match terms[..] {
        [] => panic!("Zero terms"),
        [a] => a == target,
        _ => {
            let last_term = terms[terms.len() - 1];
            if last_term > target {
                false
            } else if target % last_term == 0 {
                valid1(target / last_term, &terms[..(terms.len() - 1)]) ||
                valid1(target - last_term, &terms[..(terms.len() - 1)])
            } else {
                valid1(target - last_term, &terms[..(terms.len() - 1)])
            }
        }
    }
}

fn valid2(target: u64, terms: &[u64]) -> bool {
    match terms[..] {
        [] => panic!("Zero terms"),
        [a] => a == target,
        _ => {
            let last_term = terms[terms.len() - 1];
            if last_term > target {
                false
            } else {
                let by_add = valid2(target - last_term, &terms[..(terms.len() - 1)]);
                let by_mult = target % last_term == 0 &&
                    valid2(target / last_term, &terms[..(terms.len() - 1)]);
                let last_term_digits = u32::try_from(last_term.to_string().len()).unwrap();
                let by_concat = (target - last_term) % 10_u64.pow(last_term_digits) == 0 &&
                    valid2((target - last_term) / 10_u64.pow(last_term_digits), &terms[..(terms.len() - 1)]);
                
                by_add || by_mult || by_concat
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 3749);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 11387);
    }
}
