impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if (matrix[i][j] != 0 && i > 0 && j > 0) {
                    matrix[i][j] +=
                        matrix[i - 1][j - 1].min(matrix[i - 1][j].min(matrix[i][j - 1]));
                }
                ans += matrix[i][j];
            }
        }
        ans
    }
}
