use crate::day10::{Data, Input, Inst::*, Output};

pub fn solve(input: &Input) -> Output {
    let mut total = 0;
    let mut regx = 1;
    let mut cycle = 0;

    for i in 0..input.len() {
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            // println!("{},{}", regx, cycle);
            total += regx * cycle;
        }
        if input[i].instruction == ADDX {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                // println!("{},{}", regx, cycle);
                total += regx * cycle;
            }
            regx += input[i].value;
        }
    }

    total.into()
}
