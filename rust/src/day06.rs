use std::fs;
use std::collections::{ HashMap, HashSet };

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day06.txt")
        .expect("Error reading file")
}

enum Orientation {
    North,
    South,
    West,
    East
}

fn problem1_str(data: String) -> usize {
    let entries = data
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, ch)| ((row, col), ch) )
        });

    let mut board = HashMap::new();
    let (mut cur_r, mut cur_c, mut orientation): (usize, usize, Orientation) = (0, 0, Orientation::North);
    for ((row, col), ch) in entries {
        if ch == '^' {
            // assuming we'll only start north
            (cur_r, cur_c) = (row, col);
            orientation = Orientation::North;
            board.insert((row, col), '.');
        } else {
            board.insert((row, col), ch);
        }
    }

    let mut visited = HashSet::new();
    while board.contains_key(&(cur_r, cur_c)) {
        visited.insert((cur_r, cur_c));
        if matches!(orientation, Orientation::North) && cur_r == 0 {
            break;
        }
        if matches!(orientation, Orientation::West) && cur_c == 0 {
            break;
        }
        let next_pos = match orientation {
            Orientation::North => (cur_r - 1, cur_c),
            Orientation::South => (cur_r + 1, cur_c),
            Orientation::West => (cur_r, cur_c - 1),
            Orientation::East => (cur_r, cur_c + 1),
        };
        if let Some('#') = board.get(&next_pos) {
            orientation = match orientation {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            }
        } else {
            (cur_r, cur_c) = next_pos;
        }
    }
    visited.len()
}

fn problem2_str(data: String) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 41);
    }

    #[ignore]
    #[rstest]
    fn problem2_test(input1: String) {
        unimplemented!();
    }
}
