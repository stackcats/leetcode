impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let layer = (m / 2).min(n / 2);
        for l in 0..layer {
            let mut v = Vec::new();
            for i in l..(n - l) {
                v.push(grid[l][i]);
            }

            for i in (l + 1)..(m - l) {
                v.push(grid[i][n - l - 1]);
            }

            for i in (l..(n - l - 1)).rev() {
                v.push(grid[m - l - 1][i]);
            }

            for i in ((l + 1)..(m - l - 1)).rev() {
                v.push(grid[i][l]);
            }

            let offset = k % v.len();
            v.rotate_left(offset);
            let mut j = 0;
            for i in l..(n - l) {
                grid[l][i] = v[j];
                j += 1;
            }

            for i in (l + 1)..(m - l) {
                grid[i][n - l - 1] = v[j];
                j += 1;
            }

            for i in (l..(n - l - 1)).rev() {
                grid[m - l - 1][i] = v[j];
                j += 1;
            }

            for i in ((l + 1)..(m - l - 1)).rev() {
                grid[i][l] = v[j];
                j += 1;
            }
        }

        grid
    }
}
