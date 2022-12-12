use std::collections::VecDeque;

use crate::day12::{Data, Input, Output, Ty::*};
pub fn solve(input: &Input) -> Output {
    let mut start: Vec<(usize, usize)> = Vec::new();
    let mut end: (usize, usize) = (0, 0);
    let mut map = vec![vec![0; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            map[i][j] = input[i][j] as i32;
            if input[i][j] == 'S' || input[i][j] == 'a' {
                start.push((i, j));
                map[i][j] = 'a' as i32;
            } else if input[i][j] == 'E' {
                end = (i, j);
                map[i][j] = 'z' as i32;
            }
        }
    }

    let mut queue = VecDeque::new();
    for i in 0..start.len() {
        queue.push_back((start[i], 0));
    }
    let mut visited = vec![vec![false; input[0].len()]; input.len()];

    let mut outlen = 0;

    while queue.len() > 0 {
        let (cur, len) = queue.pop_front().unwrap_or_default();
        // base case
        if cur == end {
            outlen = len;
            break;
        }
        for (x, y) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let newx = (cur.0 as isize + x) as usize;
            let newy = (cur.1 as isize + y) as usize;
            let Some(&check) = map.get(newx).and_then(|r| r.get(newy)) else {continue};

            if map[cur.0][cur.1] + 1 >= check && !visited[newx][newy] {
                visited[newx][newy] = true;
                queue.push_back(((newx, newy), len + 1));
            }
        }
    }

    outlen.into()
}
