impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut mat = vec![vec![0; col_sum.len()]; row_sum.len()];

        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                mat[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= mat[i][j];
                col_sum[j] -= mat[i][j];
            }
        }

        mat
    }
}
