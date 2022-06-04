fn bfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if grid[i][j] == 0 {
        return;
    }
    let mut q = Vec::new();
    q.push((i, j));
    while let Some((x, y)) = q.pop() {
        grid[x][y] = 0;
        for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == 1 {
                q.push((nx, ny));
            }
        }
    }
}

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let M = grid.len();
        let N = grid[0].len();
        for j in 0..N {
            bfs(&mut grid, 0, j);
            bfs(&mut grid, M-1, j);
        }
        for i in 0..M {
            bfs(&mut grid, i, 0);
            bfs(&mut grid, i, N-1);
        }
        grid.iter().fold(0, |acc, row| row.iter().sum::<i32>() + acc)
    }
}
