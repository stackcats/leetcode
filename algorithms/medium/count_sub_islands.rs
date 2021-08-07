use std::collections::VecDeque;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid1[0].len()]; grid1.len()];
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut ans = 0;
        for i in 0..grid2.len() {
            for j in 0..grid2[i].len() {
                if grid2[i][j] == 0 {
                    continue;
                }
                if visited[i][j] {
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
                            && (x as usize) < grid2.len()
                            && y >= 0
                            && (y as usize) < grid2[0].len()
                            && grid2[x as usize][y as usize] == 1
                        {
                            q.push_back((x as usize, y as usize));
                        }
                    }
                }
                let mut is_sub_island = true;
                for (x, y) in island.iter() {
                    if grid1[*x][*y] == 0 {
                        is_sub_island = false;
                        break;
                    }
                }
                if is_sub_island {
                    ans += 1;
                }
            }
        }
        ans
    }
}
