use std::env;
use itertools::Itertools;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        day01::problem2();
    } else {
        let problem = args[1..]
            .into_iter()
            .map(|x| x.parse::<i8>().unwrap())
            .collect_tuple();
        match problem {
            Some((1, 1)) => day01::problem1(),
            Some((1, 2)) => day01::problem2(),
            _ => panic!("Unrecognized problem arguments {:?}", args)
        }
    }
}
