fn dfs(m: i32, n: i32, x: i32, y: i32, moves: i32, dp: &mut [Vec<Vec<i32>>]) -> i32 {
    let is_out = x < 0 || x >= m || y < 0 || y >= n;
    if is_out {
        return 1;
    }
    if moves == 0 {
        return 0;
    }
    if dp[x as usize][y as usize][moves as usize] != -1 {
        return dp[x as usize][y as usize][moves as usize];
    }
    dp[x as usize][y as usize][moves as usize] = 0;
    let dirs = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    for (dx, dy) in &dirs {
        dp[x as usize][y as usize][moves as usize] += dfs(m, n, dx + x, dy + y, moves - 1, dp);
        dp[x as usize][y as usize][moves as usize] %= 1000000007;
    }
    dp[x as usize][y as usize][moves as usize]
}

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; 60 as usize]; 60 as usize]; 60 as usize];
        dfs(m, n, start_row, start_column, max_move, &mut dp)
    }
}
