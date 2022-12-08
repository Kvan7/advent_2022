use crate::day07::{Data, Input};
use aoc_parse::{parser, prelude::*};
use nom::ParseTo;

const INPUT: &str = include_str!("../../input/07/input.txt");

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
    let lins: Vec<&str> = INPUT.lines().collect();
    let mut out: Vec<Data> = Vec::new();
    let mut cur_dir: Vec<String> = Vec::new();
    for i in 0..lins.len() {
        let temp: Vec<&str> = lins[i].split_whitespace().collect();
        match &lins[i][0..1] {
            "$" => {
                if temp[1] == "ls" {
                    continue;
                }
                match temp[2] {
                    ".." => {
                        cur_dir.pop();
                    }
                    _ => cur_dir.push(temp[2].to_owned()),
                }
            }
            "d" => continue,
            _ => out.push(Data {
                name: temp[1].to_string(),
                parent: cur_dir.last().unwrap().to_owned(),
                size: temp[0].parse_to().unwrap(),
            }),
        }
    }
    out
}
