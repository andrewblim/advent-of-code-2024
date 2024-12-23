use std::fs;
use std::cmp;
use std::collections::{ HashMap, HashSet };
use itertools::Itertools;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day08.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    let (antennas, max_row, max_col) = parse_board(&data);
    let mut antinodes = HashSet::new();
    for (_freq, locations) in antennas.into_iter() {
        for (p1, p2) in locations.iter().tuple_combinations() {
            for loc in antinodes_for1(*p1, *p2, max_row, max_col) {
                antinodes.insert(loc);
            };
        }
    }
    antinodes.len()
}

fn problem2_str(data: String) -> usize {
    unimplemented!();
}

fn parse_board(data: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize) {
    let entries = data
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, ch)| ((row, col), ch) )
        });

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let (mut max_row, mut max_col) = (0, 0);
    for ((row, col), ch) in entries {
        if ch != '.' {
            antennas.entry(ch)
                .and_modify(|pts| { pts.push((row, col)); })
                .or_insert(vec![(row, col)]);
        }
        max_row = cmp::max(row, max_row);
        max_col = cmp::max(col, max_col);
    }
    (antennas, max_row, max_col)
}

fn antinodes_for1((r1, c1): (usize, usize), (r2, c2): (usize, usize), max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut antinodes = vec![];
    let (ra1, ra2) = if r2 > r1 {
        (
            r1.checked_sub(r2 - r1),
            if r2 + r2 - r1 > max_row { None } else { Some(r2 + r2 - r1) }
        )
    } else {
        (
            if r1 + r1 - r2 > max_row { None } else { Some(r1 + r1 - r2) },
            r2.checked_sub(r1 - r2)
        )
    };
    let (ca1, ca2) = if c2 > c1 {
        (
            c1.checked_sub(c2 - c1),
            if c2 + c2 - c1 > max_col { None } else { Some(c2 + c2 - c1) }
        )
    } else {
        (
            if c1 + c1 - c2 > max_col { None } else { Some(c1 + c1 - c2) },
            c2.checked_sub(c1 - c2)
        )
    };
    if let (Some(r), Some(c)) = (ra1, ca1) {
        antinodes.push((r, c))
    };
    if let (Some(r), Some(c)) = (ra2, ca2) {
        antinodes.push((r, c))
    };
    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 14);
    }

    #[ignore]
    #[rstest]
    fn problem2_test(input1: String) {
        unimplemented!();
    }
}
