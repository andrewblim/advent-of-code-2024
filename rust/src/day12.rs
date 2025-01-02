use std::{collections::{HashMap, HashSet}, fs, thread::current};

use itertools::Itertools;

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day12.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    let board = parse_board(&data);
    let shapes = find_shapes(&board);
    let mut price = 0;
    for shape in shapes {
        price += score_shape(&shape);
    }
    price
}

fn problem2_str(data: String) -> usize {
    let board = parse_board(&data);
    let shapes = find_shapes(&board);
    let mut price = 0;
    for shape in shapes {
        price += score_shape2(&shape);
    }
    price
}

fn parse_board(data: &str) -> HashMap<(i64, i64), char> {
    let positions = data
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as i64, col as i64), ch))
        });
    HashMap::from_iter(positions)
}

fn find_shapes(board: &HashMap<(i64, i64), char>) -> Vec<HashSet<(i64, i64)>> {
    let mut unvisited = HashSet::new();
    let mut shapes = Vec::new();
    for &(r, c) in board.keys() {
        unvisited.insert((r, c));
    }
    while unvisited.len() > 0 {
        let start = *unvisited.iter().next().unwrap();
        let shape = find_shape(&board, start);
        for &(r, c) in &shape {
            unvisited.remove(&(r, c));
        }
        shapes.push(shape);
    }
    shapes
}

fn find_shape(board: &HashMap<(i64, i64), char>, start: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut shape = HashSet::from([start]);
    let mut current = HashSet::from([start]);
    let plant = board.get(&start).unwrap();

    while current.len() > 0 {
        let mut next = HashSet::new();
        for (r, c) in current {
            for (adj_r, adj_c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if shape.contains(&(adj_r, adj_c)) {
                    continue;
                }
                if let Some(v) = board.get(&(adj_r, adj_c)) {
                    if *v == *plant {
                        shape.insert((adj_r, adj_c));
                        next.insert((adj_r, adj_c));
                    }
                }
            }
        }
        current = next;
    }
    shape
}

fn score_shape(shape: &HashSet<(i64, i64)>) -> usize {
    let area = shape.len();
    let mut perimeter = 0;
    for &(r, c) in shape {
        for (adj_r, adj_c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            if !shape.contains(&(adj_r, adj_c)) {
                perimeter += 1;
            }
        }
    }
    area * perimeter
}

fn score_shape2(shape: &HashSet<(i64, i64)>) -> usize {
    let area = shape.len();
    let mut n_sides = 0;

    let mut shape_by_row: Vec<&(i64, i64)> = Vec::from_iter(shape.into_iter());
    shape_by_row.sort();

    let mut current_sides = HashSet::new();
    for (r, chunk) in &shape_by_row.into_iter().chunk_by(|(r, _)| *r) {
        let mut next_sides = HashSet::new();
        for &(_, c) in chunk {
            for adj_c in [c - 1, c + 1] {
                if !shape.contains(&(r, adj_c)) {
                    next_sides.insert((c, adj_c));
                    if !current_sides.contains(&(c, adj_c)) {
                        n_sides += 1;
                    }
                }
            }
        }
        current_sides = next_sides;
    }

    let mut shape_by_col: Vec<&(i64, i64)> = Vec::from_iter(shape.into_iter());
    shape_by_col.sort_by_key(|(r, c)| (c, r));

    let mut current_sides = HashSet::new();
    for (c, chunk) in &shape_by_col.into_iter().chunk_by(|(_, c)| *c) {
        let mut next_sides = HashSet::new();
        for &(r, _) in chunk {
            for adj_r in [r - 1, r + 1] {
                if !shape.contains(&(adj_r, c)) {
                    next_sides.insert((r, adj_r));
                    if !current_sides.contains(&(r, adj_r)) {
                        n_sides += 1;
                    }
                }
            }
        }
        current_sides = next_sides;
    }

    area * n_sides
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
AAAA
BBCD
BBCC
EEEC
        ")
    }

    #[fixture]
    fn input2() -> String {
        String::from("\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
        ")
    }

    #[fixture]
    fn input3() -> String {
        String::from("\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        ")
    }

    #[fixture]
    fn input4() -> String {
        String::from("\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
        ")
    }

    #[fixture]
    fn input5() -> String {
        String::from("\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
        ")
    }

    #[rstest]
    fn problem1_test(input1: String, input2: String, input3: String) {
        assert_eq!(problem1_str(input1), 140);
        assert_eq!(problem1_str(input2), 772);
        assert_eq!(problem1_str(input3), 1930);
    }

    #[rstest]
    fn problem2_test(input1: String, input2: String, input3: String, input4: String, input5: String) {
        assert_eq!(problem2_str(input1), 80);
        assert_eq!(problem2_str(input2), 436);
        assert_eq!(problem2_str(input3), 1206);
        assert_eq!(problem2_str(input4), 236);
        assert_eq!(problem2_str(input5), 368);
    }
}
