impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut suff = 1;
        let mut v = vec![vec![0; grid[0].len()]; grid.len()];

        for i in (0..grid.len()).rev() {
            for j in (0..grid[i].len()).rev() {
                v[i][j] = suff as i32;
                suff = (suff * grid[i][j] as i64) % 12345;
            }
        }

        let mut prev = 1;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                v[i][j] = (prev as i32 * v[i][j]) % 12345;
                prev = (prev * grid[i][j] as i64) % 12345;
            }
        }

        v
    }
}
