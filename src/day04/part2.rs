use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut count = 0;

    for i in 0..input.len() {
        if (input[i].one.start <= input[i].two.start && input[i].one.end >= input[i].two.start)
            || (input[i].one.start >= input[i].two.start && input[i].one.start <= input[i].two.end)
            || (input[i].one.end >= input[i].two.end && input[i].one.start <= input[i].two.end)
            || (input[i].one.end <= input[i].two.end && input[i].one.end >= input[i].two.start)
        {
            count += 1;
        }
    }

    Output::from(count)
}
