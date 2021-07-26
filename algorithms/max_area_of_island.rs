use std::collections::VecDeque;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
        let mut q = VecDeque::new();
        let mut ans = 0;
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited[i][j] == 1 {
                    continue;
                }
                if grid[i][j] == 0 {
                    continue;
                }
                q.push_back((i, j));
                let mut curr_max = 0;
                while !q.is_empty() {
                    let (mut m, mut n) = q.pop_front().unwrap();
                    if visited[m][n] == 1 {
                        continue;
                    }
                    curr_max += 1;
                    visited[m][n] = 1;
                    for (dx, dy) in &dirs {
                        let x = m as i32 + dx;
                        let y = n as i32 + dy;
                        if x >= 0
                            && (x as usize) < grid.len()
                            && y >= 0
                            && (y as usize) < grid[0].len()
                            && grid[x as usize][y as usize] == 1
                        {
                            q.push_back((x as usize, y as usize));
                        }
                    }
                }
                ans = ans.max(curr_max);
            }
        }
        ans
    }
}
