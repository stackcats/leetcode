impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut top = vec![0; grid[0].len()];
        let mut left = vec![0; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if left[i] < grid[i][j] {
                    left[i] = grid[i][j];
                }
                if top[j] < grid[i][j] {
                    top[j] = grid[i][j];
                }
            }
        }
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                ans += top[j].min(left[i]) - grid[i][j];
            }
        }
        ans
    }
}
