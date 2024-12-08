use std::env;
use itertools::Itertools;

mod day01;
mod day02;
mod day03;

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
            _ => panic!("Unrecognized problem arguments {:?}", args)
        }
    }
}
