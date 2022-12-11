pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Add,
    Multiply,
    Power,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Oper {
    operation: Op,
    adden: u128,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Data {
    id: u32,
    items: Vec<u128>,
    operation: Oper,
    test: u128,
    test_true: u32,
    test_false: u32,
}

pub type Input = Vec<Data>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
