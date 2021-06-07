fn add_to_row(mat: &mut Vec<Vec<i32>>, row: usize, n: i32) {
    for j in 0..mat[0].len() {
        if mat[row][j] == -1 {
            mat[row][j] = n;
            return;
        }
    }
}

fn add_to_col(mat: &mut Vec<Vec<i32>>, col: usize, n: i32) {
    for i in 0..mat.len() {
        if mat[i][col] == -1 {
            mat[i][col] = n;
            return;
        }
    }
}

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row_sum: Vec<(usize, i32)> = row_sum.into_iter().enumerate().collect();
        let mut col_sum: Vec<(usize, i32)> = col_sum.into_iter().enumerate().collect();
        row_sum.sort_by(|(_, a), (_, b)| a.cmp(b));
        col_sum.sort_by(|(_, a), (_, b)| a.cmp(b));
        let mut ans = vec![vec![-1; col_sum.len()]; row_sum.len()];
        let mut r = 0;
        let mut c = 0;
        let mut offset = 0;
        while r < row_sum.len() && c < col_sum.len() {
            let (i, x) = row_sum[r];
            let (j, y) = col_sum[c];
            if x < y {
                add_to_row(&mut ans, i, x - offset);
                offset += x;
                r += 1;
            } else {
                add_to_col(&mut ans, j, y - offset);
                offset += y;
                c += 1;
            }
        }
        while r < row_sum.len() {
            let (i, x) = row_sum[r];

            add_to_row(&mut ans, i, x - offset);
            offset += x;
            r += 1;
        }
        while c < col_sum.len() {
            let (j, y) = col_sum[c];

            add_to_col(&mut ans, j, y - offset);
            offset += y;
            c += 1;
        }
        ans
    }
}
