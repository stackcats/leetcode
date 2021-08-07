impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut end = grid[0].len();
        for i in 0..grid.len() {
            let mut j = 0;
            while j < end && grid[i][j] >= 0 {
                j += 1;
            }
            ans += grid[i].len() - j;
            end = j;
        }
        ans as i32
    }
}
