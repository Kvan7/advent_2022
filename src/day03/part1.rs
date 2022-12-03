use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut prio: u32 = 0;
    for i in 0..input.len() {
        let one = input[i].0 .0.as_bytes();
        let two = input[i].0 .1.as_bytes();
        let mut char: u8 = 0;
        for j in 0..one.len() {
            for k in 0..two.len() {
                if one[j] == two[k] {
                    char = one[j];
                }
            }
        }
        if char.is_ascii_lowercase() {
            prio += (char - 96) as u32;
        } else {
            prio += (char - 64 + 26) as u32;
        }
    }
    Output::from(prio)
}
