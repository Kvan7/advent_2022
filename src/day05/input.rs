use std::vec;

use crate::day05::{Input, Inst};
use aoc_parse::{parser, prelude::*};

const INPUT: &str = include_str!("../../input/05/input.txt");

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
    let p = parser!(lines("move " (a: u32) " from " (b: u32) " to " (c: u32) => Inst {count: a, from: b, to: c}));

    // let ve = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];

    let v = parser!(lines(repeat_sep(alpha, ",")));

    // println!("{}", &INPUT[..11]);
    // println!("{}", &INPUT[13..]);

    // (
    //     v.parse(&INPUT[..12]).unwrap(),
    //     p.parse(&INPUT[13..]).unwrap(),
    // )

    // println!("{}", &INPUT[..112]);
    // println!("{}", &INPUT[113..]);

    (
        v.parse(&INPUT[..112]).unwrap(),
        p.parse(&INPUT[113..]).unwrap(),
    )
}
