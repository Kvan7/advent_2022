use crate::day08::Input;
use aoc_parse::{parser, prelude::*};
const INPUT: &str = include_str!("../../input/08/input.txt");

pub fn read() -> Input {
    let p = parser!(lines(digit*));
    p.parse(INPUT).unwrap()
}
