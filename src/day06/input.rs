use crate::day06::Input;
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/06/input.txt");

// let p = parser!(repeat_sep(u32, ","));
// let p = parser!(lines(u32));

// binary
// let p = parser!(lines(alnum*));
//     p.parse(INPUT)
//         .unwrap_or_default()
//         .iter()
//         .map(|s| {
//             i32::from_str_radix(s.iter().cloned().collect::<String>().as_str(), 2)
//                 .unwrap_or_default()
//         })
//         .collect()

pub fn read() -> Input {
    let p = parser!(alpha*);
    p.parse(INPUT).unwrap()
}
