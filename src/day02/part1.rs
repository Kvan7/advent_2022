use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut score = 0;

    for i in 0..input.len() {
        if input[i].op == input[i].me {
            score += input[i].me + 3;
            continue;
        } else if (input[i].me == 3 && input[i].op == 2)
            || (input[i].me == 2 && input[i].op == 1)
            || (input[i].me == 1 && input[i].op == 3)
        {
            score += input[i].me + 6;
            continue;
        } else {
            score += input[i].me;
            continue;
        }
    }
    Output::from(score)
}
