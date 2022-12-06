use crate::day06::{Input, Output};
use std::collections::HashSet;

pub fn solve(input: &Input) -> Output {
    let mut queue: Vec<char> = Vec::new();

    for i in 0..input.len() {
        // println!("{}: {:?}", i, queue);
        if queue.len() == 14 {
            let mut uniq = HashSet::new();
            if queue.iter().all(move |x| uniq.insert(x)) {
                // queue.contains(&input[i]) &&
                return (i as u32).into();
            }
            queue.pop();
        }
        queue.insert(0, input[i]);
    }

    return (0 as u32).into();
}
