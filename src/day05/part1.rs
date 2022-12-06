use crate::day05::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let instructions = &input.1;
    let mut stacks = input.0.clone();

    for i in 0..instructions.len() {
        for mov in 0..instructions[i].count {
            let temp = stacks[instructions[i].from as usize - 1]
                .pop()
                .unwrap_or_default();
            stacks[instructions[i].to as usize - 1].push(temp);
        }
    }

    let mut ans = String::new();
    for i in 0..stacks.len() {
        ans.push(stacks[i].pop().unwrap_or_default());
    }

    ans.into()
}
