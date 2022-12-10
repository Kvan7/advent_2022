use crate::day10::{Data, Input, Inst::*, Output};

pub fn solve(input: &Input) -> Output {
    let mut total: i32 = 0;
    let mut regx: i32 = 1;
    let mut cycle: i32 = 0;

    let mut crt = vec![vec!['.'; 40]; 6];

    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        crt[0], crt[1], crt[2], crt[3], crt[4], crt[5]
    );

    for i in 0..input.len() {
        if regx == cycle % 40 || (regx + 1 == (cycle % 40) || regx - 1 == (cycle % 40)) {
            crt[(cycle / 40) as usize][(cycle % 40) as usize] = '#';
        }
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            println!("{},{}", regx, cycle);

            total += regx * cycle;
        }
        if input[i].instruction == ADDX {
            if regx == cycle % 40 || (regx + 1 == (cycle % 40) || regx - 1 == (cycle % 40)) {
                crt[(cycle / 40) as usize][(cycle % 40) as usize] = '#';
            }
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                println!("{},{}", regx, cycle);
                total += regx * cycle;
            }
            regx += input[i].value;
        }
    }

    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        crt[0], crt[1], crt[2], crt[3], crt[4], crt[5]
    );

    total.into()
}
