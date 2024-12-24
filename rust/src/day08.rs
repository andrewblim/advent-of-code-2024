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
            for loc in antinodes_for1(*p2, *p1, max_row, max_col) {
                antinodes.insert(loc);
            };
        }
    }
    antinodes.len()
}

fn problem2_str(data: String) -> usize {
    let (antennas, max_row, max_col) = parse_board(&data);
    let mut antinodes = HashSet::new();
    for (_freq, locations) in antennas.into_iter() {
        for (p1, p2) in locations.iter().tuple_combinations() {
            for loc in antinodes_for2(*p1, *p2, max_row, max_col) {
                antinodes.insert(loc);
            };
            for loc in antinodes_for2(*p2, *p1, max_row, max_col) {
                antinodes.insert(loc);
            };
        }
    }
    antinodes.len()
}

fn parse_board(data: &str) -> (HashMap<char, Vec<(i64, i64)>>, i64, i64) {
    let entries = data
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, ch)| ((row as i64, col as i64), ch) )
        });

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
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

fn antinodes_for1((r1, c1): (i64, i64), (r2, c2): (i64, i64), max_row: i64, max_col: i64) -> Vec<(i64, i64)> {
    let mut antinodes = vec![];
    let ra = r2 + r2 - r1;
    let ca = c2 + c2 - c1;
    if ra >= 0 && ra <= max_row && ca >= 0 && ca <= max_col {
        antinodes.push((ra, ca))
    };
    antinodes
}

fn antinodes_for2((r1, c1): (i64, i64), (r2, c2): (i64, i64), max_row: i64, max_col: i64) -> Vec<(i64, i64)> {
    let mut antinodes = vec![];
    let rdiff = r2 - r1;
    let cdiff = c2 - c1;
    let mut ra = r2;
    let mut ca = c2;
    while ra >= 0 && ra <= max_row && ca >= 0 && ca <= max_col {
        antinodes.push((ra, ca));
        ra = ra + rdiff;
        ca = ca + cdiff;
    }
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

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 34);
    }
}
