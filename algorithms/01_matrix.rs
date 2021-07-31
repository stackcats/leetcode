impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![std::i32::MAX - 1; mat[0].len()]; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 0 {
                    ans[i][j] = 0;
                    continue;
                }
                if i > 0 {
                    ans[i][j] = ans[i][j].min(ans[i - 1][j] + 1);
                }
                if j > 0 {
                    ans[i][j] = ans[i][j].min(ans[i][j - 1] + 1);
                }
            }
        }

        for i in (0..mat.len()).rev() {
            for j in (0..mat[i].len()).rev() {
                if i < mat.len() - 1 {
                    ans[i][j] = ans[i][j].min(ans[i + 1][j] + 1);
                }
                if j < mat[i].len() - 1 {
                    ans[i][j] = ans[i][j].min(ans[i][j + 1] + 1);
                }
            }
        }

        ans
    }
}
