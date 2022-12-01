use std::collections::HashMap;

use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut max = 0;
    let mut elf = 0;

    for i in 0..input.len() {
        let j: u32 = input[i]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .sum();
        if j > max {
            elf = i as i32;
            max = j;
        }
    }

    return Output::from(max);
}
