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

fn problem1_str(data: String) -> i64 {
    let eqns = data
        .trim()
        .split("\n")
        .map(|line| {
            let (result_str, terms_str) = line.split_once(':').unwrap();
            let terms: Vec<i64> = terms_str
                .trim()
                .split_whitespace()
                .map(|term| term.parse::<i64>().unwrap())
                .collect();
            (result_str.parse::<i64>().unwrap(), terms)
        });
    
    eqns
        .filter(|(result, terms) | valid(*result, terms))
        .map(|(result, _)| result)
        .sum()
}

fn problem2_str(data: String) -> i64 {
    unimplemented!();
}

fn valid(target: i64, terms: &[i64]) -> bool {
    match terms[..] {
        [] => panic!("Zero terms"),
        [a] => a == target,
        _ => {
            let last_term = terms[terms.len() - 1];
            if target % last_term == 0 {
                valid(target / last_term, &terms[..(terms.len() - 1)]) ||
                valid(target - last_term, &terms[..(terms.len() - 1)])
            } else {
                valid(target - last_term, &terms[..(terms.len() - 1)])
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

    #[ignore]
    #[rstest]
    fn problem2_test(input1: String) {
        unimplemented!();
    }
}
