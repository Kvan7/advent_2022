use crate::day04::{Elf, Grp, Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut count = 0;

    for i in 0..input.len() {
        if (input[i].one.start <= input[i].two.start && input[i].one.end >= input[i].two.end)
            || (input[i].one.start >= input[i].two.start && input[i].one.end <= input[i].two.end)
        {
            count += 1;
        }
    }

    Output::from(count)
}
