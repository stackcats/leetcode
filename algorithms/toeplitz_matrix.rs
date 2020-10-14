impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut row = &matrix[0];
        for i in 1..matrix.len() {
            for j in 0..(row.len() - 1) {
                if matrix[i][j + 1] != row[j] {
                    return false;
                }
            }
            row = &matrix[i];
        }
        true
    }
}
