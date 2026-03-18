impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        for i in 0..grid.len() {
            for j in 1..grid[i].len() {
                grid[i][j] += grid[i][j - 1];
            }
        }

        let mut ans = 0;

        for j in 0..grid[0].len() {
            let mut sum = 0;
            for i in 0..grid.len() {
                sum += grid[i][j];
                if sum > k {
                    break;
                }
                ans += 1;
            }
        }

        ans
    }
}
