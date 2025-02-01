impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![0; n as usize]; m as usize];

        for w in walls {
            grid[w[0] as usize][w[1] as usize] = 1;
        }

        for g in &guards {
            grid[g[0] as usize][g[1] as usize] = 1;
        }

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for g in guards {
            for (dx, dy) in &dirs {
                let mut x = g[0] + dx;
                let mut y = g[1] + dy;
                while x >= 0 && x < m && y >= 0 && y < n && grid[x as usize][y as usize] != 1 {
                    grid[x as usize][y as usize] = 2;
                    x += dx;
                    y += dy;
                }
            }
        }

        let mut ct = 0;
        for r in grid {
            ct += r.into_iter().filter(|&v| v == 0).count() as i32;
        }
        ct
    }
}
