impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len() - 2;
        let mut ans = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for di in 0..3 {
                    for dj in 0..3 {
                        ans[i][j] = ans[i][j].max(grid[i + di][j + dj]);
                    }
                }
            }
        }

        ans
    }
}
