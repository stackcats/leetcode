impl Solution {
    pub fn minimum_operations(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ct = 0;
        for j in 0..grid[0].len() {
            for i in 1..grid.len() {
                if grid[i][j] <= grid[i - 1][j] {
                    let diff = grid[i - 1][j] - grid[i][j] + 1;
                    ct += diff;
                    grid[i][j] += diff;
                }
            }
        }
        ct
    }
}
