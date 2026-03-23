impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let mut ma = vec![vec![0; grid[0].len()]; grid.len()];
        let mut mi = vec![vec![0; grid[0].len()]; grid.len()];

        ma[0][0] = grid[0][0] as i64;
        mi[0][0] = grid[0][0] as i64;
        for j in 1..grid[0].len() {
            ma[0][j] = ma[0][j - 1] * grid[0][j] as i64;
            mi[0][j] = ma[0][j];
        }
        for i in 1..grid.len() {
            ma[i][0] = ma[i - 1][0] * grid[i][0] as i64;
            mi[i][0] = ma[i][0];
        }
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                let n = grid[i][j] as i64;
                let ma_prev = ma[i - 1][j].max(ma[i][j - 1]);
                let mi_prev = mi[i - 1][j].min(mi[i][j - 1]);
                if n >= 0 {
                    ma[i][j] = ma_prev * n;
                    mi[i][j] = mi_prev * n;
                } else {
                    ma[i][j] = mi_prev * n;
                    mi[i][j] = ma_prev * n;
                }
            }
        }

        let &ans = ma.last().unwrap().last().unwrap();
        if ans < -1 {
            -1
        } else {
            (ans % 1000000007) as _
        }
    }
}
