use crate::day09::{Input, Move};
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/09/input.txt");

pub fn read() -> Input {
    let p = parser!(lines((a: alpha) " " (b: u32) => Move {
		dir: a,
		count: (b as u32),
	}));

    p.parse(INPUT).unwrap()
}
