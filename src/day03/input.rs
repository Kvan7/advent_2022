use crate::day03::Input;
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/03/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|s| {
            (
                (
                    s[..s.to_owned().len() / 2].to_owned(),
                    s[s.to_owned().len() / 2..].to_owned(),
                ),
                s.to_owned().len() as u32,
            )
        })
        .collect()
}
