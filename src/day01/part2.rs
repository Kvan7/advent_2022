use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;

    for i in 0..input.len() {
        let j: u32 = input[i]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .sum();
        if j > one {
            three = two;
            two = one;
            one = j;
            continue;
        }
        if j > two {
            three = two;
            two = j;
            continue;
        }
        if j > three {
            three = j;
            continue;
        }
    }

    return Output::from(one + two + three);
}
