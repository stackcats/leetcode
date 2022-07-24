impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut m = 0;
        let mut n = matrix[0].len() - 1;
        while m < matrix.len() && n < matrix[0].len() {
            if target == matrix[m][n] {
                return true;
            }
            if target < matrix[m][n] {
                n -= 1;
            } else {
                m += 1;
            }
        }
        false
    }
}
