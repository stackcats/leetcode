impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cols = vec![i32::MIN; matrix[0].len()];

        for j in 0..matrix[0].len() {
            for i in 0..matrix.len() {
                cols[j] = cols[j].max(matrix[i][j]);
            }
        }

        let mut ans = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                ans[i][j] = if matrix[i][j] >= 0 {
                    matrix[i][j]
                } else {
                    cols[j]
                };
            }
        }

        ans
    }
}
