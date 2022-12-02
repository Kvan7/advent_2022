use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut score = 0;

    for i in 0..input.len() {
        match input[i].me {
            1 => {
                score += 0;
                match input[i].op {
                    1 => score += 3,
                    2 => score += 1,
                    3 => score += 2,
                    _ => unimplemented!(),
                }
            }
            2 => {
                score += 3;
                match input[i].op {
                    1 => score += 1,
                    2 => score += 2,
                    3 => score += 3,
                    _ => unimplemented!(),
                }
            }
            3 => {
                score += 6;
                match input[i].op {
                    1 => score += 2,
                    2 => score += 3,
                    3 => score += 1,
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
    Output::from(score)
}
