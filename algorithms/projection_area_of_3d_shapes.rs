impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        for i in 0..n {
            let mut largestRow = grid[i][0];
            let mut largestColumn = grid[0][i];
            for j in 1..n {
                if largestRow < grid[i][j] {
                    largestRow = grid[i][j];
                }
                if largestColumn < grid[j][i] {
                    largestColumn = grid[j][i];
                }
            }
            ans += largestRow;
            ans += largestColumn;
        }

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
