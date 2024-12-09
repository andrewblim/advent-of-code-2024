use std::fs;
use std::collections::{HashMap, HashSet};

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day04.txt")
        .expect("Error reading file")
}

const ADJACENCIES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn problem1_str(data: String) -> u64 {
    let mut locs: HashMap<char, HashSet<(isize, isize)>> = HashMap::from([
        ('X', HashSet::new()),
        ('M', HashSet::new()),
        ('A', HashSet::new()),
        ('S', HashSet::new()),
    ]);
    for (r, line) in data.trim().split("\n").enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if let Some(pts) = locs.get_mut(&ch) {
                (*pts).insert((r.try_into().unwrap(), c.try_into().unwrap()));
            }
        }
    }
    let mut count = 0;
    for &(r, c) in locs.get(&'X').unwrap().iter() {
        for &(adj_r, adj_c) in ADJACENCIES.iter() {
            if locs.get(&'M').unwrap().contains(&(r + adj_r, c + adj_c)) &&
                locs.get(&'A').unwrap().contains(&(r + adj_r * 2, c + adj_c * 2)) &&
                locs.get(&'S').unwrap().contains(&(r + adj_r * 3, c + adj_c * 3)) {
                count += 1;
            }
        }
    }
    count
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
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 18);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 48);
    }
}
