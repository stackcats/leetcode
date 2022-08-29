fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    grid[i][j] = '0';
    for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let ni = (i as i32 + dx) as usize;
        let nj = (j as i32 + dy) as usize;
        if ni < grid.len() && nj < grid[0].len() && grid[ni][nj] == '1' {
            dfs(grid, ni, nj);
        }
    }
}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    ans += 1;
                }
            }
        }
        ans
    }
}
