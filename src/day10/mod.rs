pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

#[derive(PartialEq)]
pub enum Inst {
    NOOP,
    ADDX,
}

pub struct Data {
    cycles: usize,
    instruction: Inst,
    value: i32,
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
