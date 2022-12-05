use crate::day05::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let instructions = &input.1;
    let mut stacks = input.0.clone();

    // println!("{:?}", stacks);
    for i in 0..instructions.len() {
        let mut temp_vec: Vec<char> = Vec::new();
        for mov in 0..instructions[i].count {
            let temp = stacks[instructions[i].from as usize - 1]
                .pop()
                .unwrap_or_default();
            temp_vec.push(temp);
        }
        // println!("temp_vec: {}", temp_vec.len());
        for j in 0..temp_vec.len() {
            // print!("{},", temp_vec[temp_vec.len() - j - 1]);
            stacks[instructions[i].to as usize - 1].push(temp_vec.pop().unwrap_or_default());
        }
        // println!(
        //     "\n================================\n{:?}\n{:?}\n================================",
        //     temp_vec, stacks
        // );
    }
    println!("{:?}", stacks);

    unimplemented!()
}
