impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut arr = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let r = (i + (j + (k as usize)) / grid[i].len()) % grid.len();
                let c = (j + (k as usize)) % grid[i].len();
                arr[r][c] = grid[i][j];
            }
        }
        arr
    }
}
