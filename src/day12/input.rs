use crate::day12::{Data, Input, Ty::*};
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/12/input.txt");

pub fn read() -> Input {
    let p = parser!(
        lines(alpha*)
    );

    p.parse(INPUT).unwrap()
}
