impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if i == j || i + j + 1 == grid.len() {
                    if grid[i][j] == 0 {
                        return false;
                    }
                } else {
                    if grid[i][j] != 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}
