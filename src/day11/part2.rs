use crate::day11::{Data, Input, Op::*, Oper, Output};

pub fn solve(input: &Input) -> Output {
    let mut monkeys = input.clone();
    let mut monkey_counts = vec![0 as u128; input.len()];

    let lcm: u128 = monkeys.iter().map(|f| f.test).product();

    for i in 0..10000 {
        // loop over each round
        for j in 0..monkeys.len() {
            // loop over each monkey
            let mut monkey = monkeys[j].clone();

            monkey_counts[j] += monkey.items.len() as u128;

            for k in 0..monkey.items.len() {
                // loop over each item

                match monkey.operation.operation {
                    // perform monkey operation
                    Add => monkey.items[k] += monkey.operation.adden,
                    Multiply => monkey.items[k] *= monkey.operation.adden,
                    Power => monkey.items[k] = monkey.items[k] * monkey.items[k],
                }
                // reduce worry factor
                // monkey.items[k] /= 3 as u128;

                if monkey.items[k] % monkey.test == 0 {
                    monkeys[monkey.test_true as usize]
                        .items
                        .push(monkey.items[k] % lcm);
                } else {
                    monkeys[monkey.test_false as usize]
                        .items
                        .push(monkey.items[k] % lcm);
                }
            }
            monkey.items = vec![0 as u128; 0];
            monkeys[j] = monkey;
        }
    }
    // println!("{:?}", monkey_counts);
    monkey_counts.sort();
    monkey_counts.reverse();
    // println!("{} {}", monkey_counts[0], monkey_counts[1]);

    (monkey_counts[0] * monkey_counts[1]).into()
}
