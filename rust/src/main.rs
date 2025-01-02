use std::env;
use itertools::Itertools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        day02::problem1();
    } else {
        let problem = args[1..]
            .into_iter()
            .map(|x| x.parse::<i8>().unwrap())
            .collect_tuple();
        match problem {
            Some((1, 1)) => day01::problem1(),
            Some((1, 2)) => day01::problem2(),
            Some((2, 1)) => day02::problem1(),
            Some((2, 2)) => day02::problem2(),
            Some((3, 1)) => day03::problem1(),
            Some((3, 2)) => day03::problem2(),
            Some((4, 1)) => day04::problem1(),
            Some((4, 2)) => day04::problem2(),
            Some((5, 1)) => day05::problem1(),
            Some((5, 2)) => day05::problem2(),
            Some((6, 1)) => day06::problem1(),
            Some((6, 2)) => day06::problem2(),
            Some((7, 1)) => day07::problem1(),
            Some((7, 2)) => day07::problem2(),
            Some((8, 1)) => day08::problem1(),
            Some((8, 2)) => day08::problem2(),
            Some((9, 1)) => day09::problem1(),
            Some((9, 2)) => day09::problem2(),
            Some((10, 1)) => day10::problem1(),
            Some((10, 2)) => day10::problem2(),
            Some((11, 1)) => day11::problem1(),
            Some((11, 2)) => day11::problem2(),
            Some((12, 1)) => day12::problem1(),
            Some((12, 2)) => day12::problem2(),
            _ => panic!("Unrecognized problem arguments {:?}", args)
        }
    }
}
