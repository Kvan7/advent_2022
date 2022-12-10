use crate::day10::{Data, Input, Inst::*};
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/10/input.txt");

pub fn read() -> Input {
    let p = parser!(
        lines({"noop" => Data {cycles:1, instruction:NOOP, value:0}, "addx " (v: i32) => Data {cycles:2, instruction:ADDX, value:v}})
    );
    p.parse(INPUT).unwrap()
}
