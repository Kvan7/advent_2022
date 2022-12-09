use crate::day09::{Input, Output};
use std::collections::HashSet;

pub fn solve(input: &Input) -> Output {
    let mut knots = vec![(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..input.len() {
        // Iterate for each step
        for j in 0..input[i].count {
            match input[i].dir {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => unimplemented!(),
            }
            for k in 1..knots.len() {
                // Move Head
                // Check for tail touching head
                if knots[k].0 + 1 < knots[k - 1].0 {
                    // head right of tail
                    if knots[k].1 < knots[k - 1].1 {
                        // head in Q1
                        knots[k].1 += 1;
                    } else if knots[k].1 > knots[k - 1].1 {
                        // head in Q4
                        knots[k].1 -= 1;
                    }
                    // all Q1, Q4, and same y still increase x
                    knots[k].0 += 1;
                } else if knots[k].0 - 1 > knots[k - 1].0 {
                    if knots[k].1 < knots[k - 1].1 {
                        // head in Q2
                        knots[k].1 += 1;
                    } else if knots[k].1 > knots[k - 1].1 {
                        // head in Q3
                        knots[k].1 -= 1;
                    }
                    // all Q2, Q3, and same y still decrease x
                    knots[k].0 -= 1;
                } else if knots[k].1 + 1 < knots[k - 1].1 {
                    // head above of tail
                    if knots[k].0 < knots[k - 1].0 {
                        // head in Q1
                        knots[k].0 += 1;
                    } else if knots[k].0 > knots[k - 1].0 {
                        // head in Q2
                        knots[k].0 -= 1;
                    }
                    // all Q1, Q4, and same x still increase y
                    knots[k].1 += 1;
                } else if knots[k].1 - 1 > knots[k - 1].1 {
                    if knots[k].0 < knots[k - 1].0 {
                        // head in Q4
                        knots[k].0 += 1;
                    } else if knots[k].0 > knots[k - 1].0 {
                        // head in Q3
                        knots[k].0 -= 1;
                    }
                    // all Q4, Q3, and same x still decrease y
                    knots[k].1 -= 1;
                }
            }
            // pass tail pos to set
            // println!("{:?}, {:?}", head, tail);
            visited.insert((knots[9].0, knots[9].1));
            /*

            ..##..
            ...##.
            .####.
            ....#.
            s###..
                        */
        }
    }

    // println!("{:?}", visited);

    (visited.len() as u32).into()
}
