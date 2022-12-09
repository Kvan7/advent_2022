use crate::day09::{Input, Move, Output};
use std::collections::HashSet;

pub fn solve(input: &Input) -> Output {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..input.len() {
        // Iterate for each step
        for j in 0..input[i].count {
            // Move Head
            match input[i].dir {
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => unimplemented!(),
            }
            // Check for tail touching head
            if tail.0 + 1 < head.0 {
                // head right of tail
                if tail.1 < head.1 {
                    // head in Q1
                    tail.1 += 1;
                } else if tail.1 > head.1 {
                    // head in Q4
                    tail.1 -= 1;
                }
                // all Q1, Q4, and same y still increase x
                tail.0 += 1;
            } else if tail.0 - 1 > head.0 {
                if tail.1 < head.1 {
                    // head in Q2
                    tail.1 += 1;
                } else if tail.1 > head.1 {
                    // head in Q3
                    tail.1 -= 1;
                }
                // all Q2, Q3, and same y still decrease x
                tail.0 -= 1;
            } else if tail.1 + 1 < head.1 {
                // head above of tail
                if tail.0 < head.0 {
                    // head in Q1
                    tail.0 += 1;
                } else if tail.0 > head.0 {
                    // head in Q2
                    tail.0 -= 1;
                }
                // all Q1, Q4, and same x still increase y
                tail.1 += 1;
            } else if tail.1 - 1 > head.1 {
                if tail.0 < head.0 {
                    // head in Q4
                    tail.0 += 1;
                } else if tail.0 > head.0 {
                    // head in Q3
                    tail.0 -= 1;
                }
                // all Q4, Q3, and same x still decrease y
                tail.1 -= 1;
            }
            // pass tail pos to set
            // println!("{:?}, {:?}", head, tail);
            visited.insert((tail.0, tail.1));
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
