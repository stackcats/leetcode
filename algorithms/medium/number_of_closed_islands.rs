use std::collections::VecDeque;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut islands = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited[i][j] {
                    continue;
                }
                if grid[i][j] == 1 {
                    continue;
                }

                let mut island = Vec::new();
                let mut q = VecDeque::new();
                q.push_back((i, j));
                while !q.is_empty() {
                    let (mut m, mut n) = q.pop_front().unwrap();
                    if visited[m][n] {
                        continue;
                    }
                    visited[m][n] = true;
                    island.push((m, n));
                    for (dx, dy) in &dirs {
                        let x = m as i32 + dx;
                        let y = n as i32 + dy;
                        if x >= 0
                            && (x as usize) < grid.len()
                            && y >= 0
                            && (y as usize) < grid[0].len()
                            && grid[x as usize][y as usize] == 0
                        {
                            q.push_back((x as usize, y as usize));
                        }
                    }
                }
                islands.push(island);
            }
        }

        let mut ans = 0;
        for island in &islands {
            let mut is_closed = true;
            for &(i, j) in island.iter() {
                if i == 0 || i == grid.len() - 1 {
                    is_closed = false;
                    break;
                }
                if j == 0 || j == grid[0].len() - 1 {
                    is_closed = false;
                    break;
                }
            }
            if is_closed {
                ans += 1;
            }
        }
        ans
    }
}
