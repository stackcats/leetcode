fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, ct: &mut i32) {
    *ct += grid[i][j];
    grid[i][j] = 0;
    for (ni, nj) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
        if ni < grid.len() && nj < grid[0].len() && grid[ni][nj] > 0 {
            dfs(grid, ni, nj, ct);
        }
    }
}

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    let mut fish = 0;
                    dfs(&mut grid, i, j, &mut fish);
                    ans = ans.max(fish);
                }
            }
        }
        ans
    }
}
