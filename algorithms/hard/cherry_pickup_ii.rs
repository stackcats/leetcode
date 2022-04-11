fn dfs(grid: &[Vec<i32>], dp: &mut Vec<Vec<Vec<i32>>>, r: usize, c1: i32, c2: i32) -> i32 {
    if c1 < 0 || (c1 as usize) >= grid[0].len() || c2 < 0 || (c2 as usize) >= grid[0].len() {
        return 0;
    }

    let mut curr = if c1 == c2 {
        grid[r][c1 as usize]
    } else {
        grid[r][c1 as usize] + grid[r][c2 as usize]
    };

    if r == grid.len() - 1 {
        return curr;
    }

    if dp[r][c1 as usize][c2 as usize] > 0 {
        return dp[r][c1 as usize][c2 as usize];
    }

    if dp[r][c2 as usize][c1 as usize] > 0 {
        return dp[r][c2 as usize][c1 as usize];
    }

    let mut next = 0;
    
    for i in -1..=1 {
        for j in -1..=1 {
            next = next.max(dfs(grid, dp, r + 1, c1 + i, c2 + j));
        }
    }
    
    dp[r][c1 as usize][c2 as usize] = curr + next;
    dp[r][c2 as usize][c1 as usize] = curr + next;
    return curr + next;
}
    
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut dp = vec![vec![vec![0; c]; c]; r];
        dfs(&grid, &mut dp, 0, 0, c as i32 - 1)
    }
}
