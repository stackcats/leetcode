impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut neg_ct = 0;
        let mut ans = 0i64;
        let mut abs_min = i32::MAX;

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                ans += matrix[i][j].abs() as i64;
                if matrix[i][j] < 0 {
                    neg_ct += 1;
                }
                abs_min = matrix[i][j].abs().min(abs_min);
            }
        }

        if neg_ct % 2 == 0 {
            return ans;
        }

        ans - 2 * (abs_min as i64)
    }
}
