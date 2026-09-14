impl Solution {
    pub fn cyclic_shift(
        n: i32,
        mut grid: Vec<Vec<i32>>,
        row_shift: Vec<i32>,
        col_shift: Vec<i32>,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;

        for (i, k) in row_shift.into_iter().enumerate() {
            if k == 0 {
                continue;
            }
            let k = k as usize;
            let mut t = Vec::new();
            for j in 0..k {
                t.push(grid[i][j]);
            }
            for j in k..n {
                grid[i][j - k] = grid[i][j];
            }
            for j in 0..k {
                grid[i][n - k + j] = t[j];
            }
        }

        for (j, k) in col_shift.into_iter().enumerate() {
            if k == 0 {
                continue;
            }
            let k = k as usize;
            let mut t = Vec::new();
            for i in 0..k {
                t.push(grid[i][j]);
            }
            for i in k..n {
                grid[i - k][j] = grid[i][j];
            }
            for i in 0..k {
                grid[n - k + i][j] = t[i];
            }
        }

        grid
    }
}
