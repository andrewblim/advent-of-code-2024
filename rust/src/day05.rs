use std::fs;
use std::collections::{ HashMap, HashSet };

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day05.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> i64 {
    let (rules_text, updates_text) = data.trim().split_once("\n\n").unwrap();

    let rules = rules_text
        .split("\n")
        .map(|line| {
            let (earlier, later) = line.split_once("|").unwrap();
            ((*earlier).parse::<i64>().unwrap(), (*later).parse::<i64>().unwrap())
        })
        .fold(HashMap::new(), |mut acc: HashMap<i64, HashSet<i64>>, (earlier, later)| {
            acc.entry(later).or_default().insert(earlier);
            acc
        });

    let updates = updates_text
        .split("\n")
        .map(|line| {
            line.split(",").map(|x| (*x).parse::<i64>().unwrap()).collect::<Vec<i64>>()
        });
    
    updates
        .filter(|xs| {
            let mut disallowed: HashSet<i64> = HashSet::new();
            for x in xs {
                if disallowed.contains(&x) {
                    return false;
                }
                if rules.contains_key(&x) {
                    disallowed.extend(rules[&x].iter());
                }
            }
            true
        })
        .map(|xs| xs[xs.len() / 2])
        .sum()
}

fn problem2_str(data: String) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 143);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 9);
    }
}
