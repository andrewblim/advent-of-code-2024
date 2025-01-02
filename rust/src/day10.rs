use std::{collections::{HashMap, HashSet}, fs};

pub fn problem1() {
    println!("{}", problem1_str(read_data()));
}

pub fn problem2() {
    println!("{}", problem2_str(read_data()));
}

fn read_data() -> String {
    fs::read_to_string("data/day10.txt")
        .expect("Error reading file")
}

fn problem1_str(data: String) -> usize {
    let board = parse_board(&data);
    let mut trailheads = Vec::new();
    for ((r, c), v) in &board {
        if *v == 0 {
            trailheads.push((*r, *c));
        }
    }
    trailheads.sort();

    let mut score = 0;
    for trailhead in trailheads {
        score += score_from(trailhead, &board);
    }
    score
}

fn problem2_str(data: String) -> usize {
    let board = parse_board(&data);
    let mut trailheads = Vec::new();
    for ((r, c), v) in &board {
        if *v == 0 {
            trailheads.push((*r, *c));
        }
    }
    trailheads.sort();

    let mut rating = 0;
    for trailhead in trailheads {
        rating += rating_from(trailhead, &board);
    }
    rating
}

fn parse_board(data: &str) -> HashMap<(i64, i64), u32> {
    let positions = data
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as i64, col as i64), ch.to_digit(10).unwrap()) )
        });
    HashMap::from_iter(positions)
}

fn score_from((r, c): (i64, i64), board: &HashMap<(i64, i64), u32>) -> usize {
    let mut current = HashSet::from([(r, c)]);
    for current_elevation in (*board.get(&(r, c)).unwrap())..9 {
        let mut next_current = HashSet::new();
        for (r, c) in current {
            for (adj_r, adj_c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if let Some(v) = board.get(&(adj_r, adj_c)) {
                    if *v == current_elevation + 1 {
                        next_current.insert((adj_r, adj_c));
                    }
                }
            }
        }
        current = next_current;
    }
    current.len()
}

fn rating_from((r, c): (i64, i64), board: &HashMap<(i64, i64), u32>) -> usize {
    let mut current = HashSet::from([(r, c)]);
    let mut ways = HashMap::from([((r, c), 1)]);

    for current_elevation in (*board.get(&(r, c)).unwrap())..9 {
        let mut next_current = HashSet::new();
        for (r, c) in current {
            let ways_here = ways.get(&(r, c)).unwrap().clone();
            for (adj_r, adj_c) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if let Some(v) = board.get(&(adj_r, adj_c)) {
                    if *v == current_elevation + 1 {
                        next_current.insert((adj_r, adj_c));
                        ways.entry((adj_r, adj_c))
                            .and_modify(|e| *e += ways_here)
                            .or_insert(ways_here);
                    }
                }
            }
        }
        current = next_current;
    }

    let mut rating = 0;
    for (r, c) in current {
        rating += ways.get(&(r, c)).unwrap();
    }
    rating
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn input1() -> String {
        String::from("\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        ")
    }

    #[rstest]
    fn problem1_test(input1: String) {
        assert_eq!(problem1_str(input1), 36);
    }

    #[rstest]
    fn problem2_test(input1: String) {
        assert_eq!(problem2_str(input1), 81);
    }
}
