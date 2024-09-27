impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut d1 = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() / 2 {
                if grid[i][j] != grid[i][grid[0].len() - 1 - j] {
                    d1 += 1;
                }
            }
        }
        let mut d2 = 0;
        for j in 0..grid[0].len() {
            for i in 0..grid.len() / 2 {
                if grid[i][j] != grid[grid.len() - i - 1][j] {
                    d2 += 1;
                }
            }
        }
        d1.min(d2)
    }
}
