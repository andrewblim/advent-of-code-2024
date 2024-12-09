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

fn problem1_str(data: String) -> u64 {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let locs = point_map(&data, &['X', 'M', 'A', 'S']);
    let mut count = 0;
    for &start in locs.get(&'X').unwrap().iter() {
        for &dir in DIRECTIONS.iter() {
            if has_xmas(&locs, start, dir) {
                count += 1;
            }
        }
    }
    count
}

fn problem2_str(data: String) -> u64 {
    let locs = point_map(&data, &['M', 'A', 'S']);
    let mut count = 0;
    for &start in locs.get(&'A').unwrap().iter() {
        if has_cross_around(&locs, start) {
            count += 1;
        }
    }
    count
}

fn point_map(data: &str, chars: &[char]) -> HashMap<char, HashSet<(isize, isize)>> {
    let mut locs: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();
    for ch in chars {
        locs.insert(*ch, HashSet::new());
    }
    for (r, line) in data.trim().split("\n").enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if let Some(pts) = locs.get_mut(&ch) {
                (*pts).insert((r.try_into().unwrap(), c.try_into().unwrap()));
            }
        }
    }
    locs
}

fn has_xmas(locs: &HashMap<char, HashSet<(isize, isize)>>, (r, c): (isize, isize), (dir_r, dir_c): (isize, isize)) -> bool {
    // assumes we start at an 'X'
    locs.get(&'M').unwrap().contains(&(r + dir_r, c + dir_c)) &&
        locs.get(&'A').unwrap().contains(&(r + dir_r * 2, c + dir_c * 2)) &&
        locs.get(&'S').unwrap().contains(&(r + dir_r * 3, c + dir_c * 3))
}

fn has_cross_around(locs: &HashMap<char, HashSet<(isize, isize)>>, (r, c): (isize, isize)) -> bool {
    // assumes we start at an 'A'
    let relevant_ch = |pt| -> char {
        if locs.get(&'M').unwrap().contains(&pt) {
            'M'
        } else if locs.get(&'S').unwrap().contains(&pt) {
            'S'
        } else {
            '.'
        }
    };
    let nw = relevant_ch((r - 1, c - 1));
    let ne = relevant_ch((r - 1, c + 1));
    let sw = relevant_ch((r + 1, c - 1));
    let se = relevant_ch((r + 1, c + 1));
    (nw, ne, se, sw) == ('M', 'M', 'S', 'S') ||
        (nw, ne, se, sw) == ('M', 'S', 'S', 'M') ||
        (nw, ne, se, sw) == ('S', 'S', 'M', 'M') ||
        (nw, ne, se, sw) == ('S', 'M', 'M', 'S')
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
        assert_eq!(problem2_str(input1), 9);
    }
}
