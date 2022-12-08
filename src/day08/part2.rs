use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut max_count = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            let mut count = 0;
            let mut tot_count = 0;

            let cur = input[i][j];
            let mut height_check = 0;

            for k in (0..i).rev() {
                count += 1;
                if input[k][j] >= cur {
                    break;
                }
            }
            tot_count += count;
            count = 0;
            height_check = 0;
            for k in (i + 1)..input.len() {
                count += 1;
                if input[k][j] >= cur {
                    break;
                }
            }
            tot_count *= count;
            count = 0;
            height_check = 0;
            for k in (j + 1)..input.len() {
                count += 1;
                if input[i][k] >= cur {
                    break;
                }
            }
            tot_count *= count;
            count = 0;
            height_check = 0;
            for k in (0..j).rev() {
                count += 1;
                if input[i][k] >= cur {
                    break;
                }
            }
            tot_count *= count;
            count = 0;
            if tot_count > max_count {
                max_count = tot_count;
            }
        }
    }

    max_count.into()
}
