fn dfs(mat: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    if dp[x][y] != 0 {
        return dp[x][y];
    }
    for (dx, dy) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;
        if nx >= mat.len() || ny >= mat[0].len() {
            continue;
        }
        if mat[nx][ny] > mat[x][y] {
            dp[x][y] = dp[x][y].max(dfs(mat, dp, nx, ny));
        }        
    }
    dp[x][y] += 1;
    dp[x][y]
}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                ans = ans.max(dfs(&matrix, &mut dp, i, j))
            }
        }
        ans
    }
}
