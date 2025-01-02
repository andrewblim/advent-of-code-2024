use std::{collections::{HashMap, HashSet}, fs};

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
    unimplemented!();
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

    #[rstest]
    fn problem1_test(input1: String, input2: String, input3: String) {
        assert_eq!(problem1_str(input1), 140);
        assert_eq!(problem1_str(input2), 772);
        assert_eq!(problem1_str(input3), 1930);
    }

    #[ignore]
    #[rstest]
    fn problem2_test(input1: String) {
        unimplemented!();
    }
}
