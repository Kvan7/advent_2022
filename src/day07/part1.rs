use std::collections::HashMap;

use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut total = 0;
    let mut last_dir = String::new();
    let mut local_total = 0;

    for i in 0..input.len() {
        println!(
            "{}: size={}, parent={} |||| {}:{}",
            input[i].name, input[i].size, input[i].parent, last_dir, local_total
        );
        if input[i].size == 0 || input[i].size > 100000 {
            continue;
        } else {
            if last_dir != input[i].parent {
                last_dir = input[i].parent.clone();
                if local_total < 100000 {
                    total += local_total;
                }
                local_total = input[i].size;
            } else {
                local_total += input[i].size;
            }
        }
    }
    if local_total < 100000 {
        total += local_total;
    }
    total.into()
}
