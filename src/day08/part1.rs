use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut check = vec![vec![0; input.len()]; input.len()];

    let mut top = vec![0; input.len()];
    let mut left = vec![0; input.len()];
    let mut right = vec![0; input.len()];
    let mut bot = vec![0; input.len()];

    for i in 0..input.len() {
        for j in 0..input.len() {
            if left[i] < input[i][j] + 1 {
                left[i] = input[i][j] + 1;
                check[i][j] = 1;
            }
            if top[j] < input[i][j] + 1 {
                top[j] = input[i][j] + 1;
                check[i][j] = 1;
            }
        }
    }

    for i in (0..input.len()).rev() {
        for j in (0..input.len()).rev() {
            if right[i] < input[i][j] + 1 {
                right[i] = input[i][j] + 1;
                check[i][j] = 1;
            }
            if bot[j] < input[i][j] + 1 {
                bot[j] = input[i][j] + 1;
                check[i][j] = 1;
            }
        }
    }

    // println!("{:?}", input);
    // println!("{:?}", check);

    let mut count = 0;
    for i in 0..input.len() {
        count += check[i].iter().sum::<i32>();
    }
    count.into()
}
