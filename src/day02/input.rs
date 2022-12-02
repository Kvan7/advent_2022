use crate::day02::Data;
use crate::day02::Input;

const INPUT: &str = include_str!("../../input/02/input.txt");

fn to_in(i: &str) -> i32 {
    match i {
        "A" | "X" => return 1,
        "B" | "Y" => return 2,
        "C" | "Z" => return 3,
        _ => unimplemented!(),
    }
}

pub fn read() -> Input {
    INPUT
        .split("\n")
        .map(|s| Data {
            op: to_in(s.split(" ").nth(0).unwrap()),
            me: to_in(s.split(" ").nth(1).unwrap()),
        })
        .collect()
}
