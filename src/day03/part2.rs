use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut prio: u32 = 0;

    for i in 0..input.len() / 3 {
        let u = i * 3;
        let one = input[u].0 .0.to_owned() + input[u].0 .1.as_str();
        let two = input[u + 1].0 .0.to_owned() + input[u + 1].0 .1.as_str();
        let three = input[u + 2].0 .0.to_owned() + input[u + 2].0 .1.as_str();
        let mut done = false;
        for a in one.chars() {
            for b in two.chars() {
                if a == b {
                    for c in three.chars() {
                        if !done && b == c {
                            if c.is_ascii_lowercase() {
                                prio += (c as u8 - 96) as u32;
                            } else {
                                prio += (c as u8 - 64 + 26) as u32;
                            }
                            done = true;
                            break;
                        }
                    }
                }
                if done {
                    break;
                }
            }
            if done {
                break;
            }
        }
    }

    Output::from(prio)
}
