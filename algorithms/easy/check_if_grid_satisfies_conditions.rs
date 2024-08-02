impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i + 1 < grid.len() && grid[i][j] != grid[i + 1][j]
                    || j + 1 < grid[i].len() && grid[i][j] == grid[i][j + 1]
                {
                    return false;
                }
            }
        }
        true
    }
}
