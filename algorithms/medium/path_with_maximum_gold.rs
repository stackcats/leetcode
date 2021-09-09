use std::collections::VecDeque;

fn dfs(
    grid: &[Vec<i32>],
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
    mut curr_sum: i32,
    max_sum: &mut i32,
) {
    visited[x][y] = true;
    curr_sum += grid[x][y];
    *max_sum = (*max_sum).max(curr_sum);

    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    for &(dx, dy) in &dirs {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0
            && (nx as usize) < grid.len()
            && ny >= 0
            && (ny as usize) < grid[0].len()
            && !visited[nx as usize][ny as usize]
            && grid[nx as usize][ny as usize] != 0
        {
            dfs(grid, nx as usize, ny as usize, visited, curr_sum, max_sum);
        }
    }
    visited[x][y] = false;
}

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
                dfs(&grid, i, j, &mut visited, 0, &mut ans);
            }
        }
        ans
    }
}
