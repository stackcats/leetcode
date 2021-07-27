use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut zeros = Vec::new();
        let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut labeled_islands = vec![vec![(0, 0); grid[0].len()]; grid.len()];
        let mut label = 1;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    zeros.push((i, j));
                    continue;
                }

                if visited[i][j] {
                    continue;
                }

                let mut islands = Vec::new();
                let mut q = VecDeque::new();
                q.push_back((i, j));
                while !q.is_empty() {
                    let (mut m, mut n) = q.pop_front().unwrap();
                    if visited[m][n] {
                        continue;
                    }
                    visited[m][n] = true;
                    islands.push((m, n));
                    for (dx, dy) in &dirs {
                        let x = m as i32 + dx;
                        let y = n as i32 + dy;
                        if x >= 0
                            && (x as usize) < grid.len()
                            && y >= 0
                            && (y as usize) < grid[0].len()
                            && grid[x as usize][y as usize] != 0
                        {
                            q.push_back((x as usize, y as usize));
                        }
                    }
                }
                for (m, n) in islands.iter() {
                    labeled_islands[*m][*n] = (label, islands.len() as i32);
                }
                if islands.len() > 0 {
                    label += 1;
                }
            }
        }
        if zeros.len() == 0 {
            return (grid.len() * grid[0].len()) as i32;
        }
        let mut ans = 0;
        for (i, j) in zeros.into_iter() {
            let mut joined = HashSet::new();
            let mut sum = 1;
            for (dx, dy) in &dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x >= 0 && (x as usize) < grid.len() && y >= 0 && (y as usize) < grid[0].len() {
                    let (label, ct) = labeled_islands[x as usize][y as usize];
                    if joined.contains(&label) {
                        continue;
                    }
                    joined.insert(label);
                    sum += ct;
                }
            }
            ans = ans.max(sum);
        }
        ans
    }
}
