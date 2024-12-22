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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Orientation {
    North,
    South,
    West,
    East
}

fn problem1_str(data: String) -> usize {
    let (board, mut cur_r, mut cur_c, mut orientation) = parse_board(&data);
    let max_r = board.keys().map(|(_, c)| *c).max().unwrap();
    let max_c = board.keys().map(|(r, _)| *r).max().unwrap();

    let mut visited = HashSet::new();
    while board.contains_key(&(cur_r, cur_c)) {
        visited.insert((cur_r, cur_c));
        if next_out_of_bounds((cur_r, cur_c, orientation), (max_r, max_c)) {
            break;
        }
        (cur_r, cur_c, orientation) = step(&board, (cur_r, cur_c, orientation));
    }
    visited.len()
}

fn problem2_str(data: String) -> usize {
    let (board, orig_r, orig_c, orig_orientation) = parse_board(&data);
    let max_r = board.keys().map(|(_, c)| *c).max().unwrap();
    let max_c = board.keys().map(|(r, _)| *r).max().unwrap();

    let (mut cur_r, mut cur_c, mut cur_orientation) = (orig_r, orig_c, orig_orientation);
    let mut block_candidates = HashSet::new();
    while board.contains_key(&(cur_r, cur_c)) {
        block_candidates.insert((cur_r, cur_c));
        if next_out_of_bounds((cur_r, cur_c, cur_orientation), (max_r, max_c)) {
            break;
        }
        (cur_r, cur_c, cur_orientation) = step(&board, (cur_r, cur_c, cur_orientation));
    }
    block_candidates.remove(&(orig_r, orig_c));

    let mut blocks = HashSet::new();
    let mut board = board;
    for (block_r, block_c) in block_candidates.iter() {
        let (mut cur_r, mut cur_c, mut cur_orientation) = (orig_r, orig_c, orig_orientation);
        let mut visited = HashSet::new();
        board.insert((*block_r, *block_c), '#');
        while board.contains_key(&(cur_r, cur_c)) {
            visited.insert((cur_r, cur_c, cur_orientation));
            if next_out_of_bounds((cur_r, cur_c, cur_orientation), (max_r, max_c)) {
                break;
            }
            (cur_r, cur_c, cur_orientation) = step(&board, (cur_r, cur_c, cur_orientation));
            if visited.contains(&(cur_r, cur_c, cur_orientation)) {
                blocks.insert((block_r, block_c));
                break;
            }
        }
        board.insert((*block_r, *block_c), '.');
    }
    blocks.len()
}

fn parse_board(data: &str) -> (HashMap<(usize, usize), char>, usize, usize, Orientation) {
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

    (board, cur_r, cur_c, orientation)
}

fn next_out_of_bounds((cur_r, cur_c, orientation): (usize, usize, Orientation), (max_r, max_c): (usize, usize)) -> bool {
    (matches!(orientation, Orientation::North) && cur_r == 0) ||
        (matches!(orientation, Orientation::South) && cur_r == max_r) ||
        (matches!(orientation, Orientation::West) && cur_c == 0) ||
        (matches!(orientation, Orientation::East) && cur_c == max_c)
}

fn step(board: &HashMap<(usize, usize), char>, (cur_r, cur_c, orientation): (usize, usize, Orientation)) -> (usize, usize, Orientation) {
    let (next_r, next_c) = match orientation {
        Orientation::North => (cur_r - 1, cur_c),
        Orientation::South => (cur_r + 1, cur_c),
        Orientation::West => (cur_r, cur_c - 1),
        Orientation::East => (cur_r, cur_c + 1),
    };
    if let Some('#') = board.get(&(next_r, next_c)) {
        let next_orientation = match orientation {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
        return (cur_r, cur_c, next_orientation);
    } else {
        return (next_r, next_c, orientation);
    }
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

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 6);
    }
}
