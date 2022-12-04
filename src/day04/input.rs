use crate::day04::Elf;
use crate::day04::Grp;
use crate::day04::Input;
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/04/input.txt");

pub fn read() -> Input {
    let p = parser!(lines((start: u32) "-" (end: u32) "," (start1: u32) "-" (end1: u32) => Grp {one:Elf{start: start, end: end},
	 two: Elf{start: start1, end: end1}}));
    p.parse(INPUT).unwrap()
}
