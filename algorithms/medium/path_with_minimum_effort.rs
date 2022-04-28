use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let R = heights.len();
        let C = heights[0].len();
        let mut dp = vec![vec![i32::MAX; C]; R];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, 0, 0)));
        loop {
            let Reverse((d, x, y)) = pq.pop().unwrap();
            if (x, y) == (R - 1, C - 1) {
                return d;
            }

            if d > dp[x][y] {
                continue;
            }

            for (dx, dy) in &[(1, 0), (0, -1), (-1, 0), (0, 1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if (nx as usize) < R && (ny as usize) < C {
                    let effort = (heights[x][y] - heights[nx as usize][ny as usize])
                        .abs()
                        .max(d);
                    if dp[nx as usize][ny as usize] > effort {
                        dp[nx as usize][ny as usize] = effort;
                        pq.push(Reverse((effort, nx as usize, ny as usize)));
                    }
                }
            }
        }
    }
}
