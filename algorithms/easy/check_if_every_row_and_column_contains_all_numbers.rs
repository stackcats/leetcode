impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut row = vec![false; n];
            let mut col = vec![false; n];
            for j in 0..n {
                row[matrix[i][j] as usize - 1] = true;
                col[matrix[j][i] as usize - 1] = true;
            }

            for k in 0..n {
                if !row[k] || !col[k] {
                    return false;
                }
            }
        }

        true
    }
}
