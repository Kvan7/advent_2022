pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub struct Data {
    a: Vec<String>,
    b: Vec<String>,
}

pub struct Inst {
    count: u32,
    from: u32,
    to: u32,
}

pub type Input = (Vec<Vec<char>>, Vec<Inst>);

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
