impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut cum_mat = vec![vec![0; mat[0].len()]; mat.len()];
        cum_mat[0][0] = mat[0][0];
        for j in 1..mat[0].len() {
            cum_mat[0][j] = cum_mat[0][j - 1] + mat[0][j];
        }
        for i in 1..mat.len() {
            cum_mat[i][0] = cum_mat[i - 1][0] + mat[i][0];
        }
        for i in 1..mat.len() {
            for j in 1..mat[i].len() {
                cum_mat[i][j] =
                    cum_mat[i - 1][j] + cum_mat[i][j - 1] - cum_mat[i - 1][j - 1] + mat[i][j];
            }
        }

        let mut ans = vec![vec![0; mat[0].len()]; mat.len()];
        for i in 0..ans.len() {
            for j in 0..ans[i].len() {
                let x = (i + k as usize).min(mat.len() - 1);
                let y = (j + k as usize).min(mat[i].len() - 1);
                ans[i][j] = cum_mat[x][y];
                let left_mat_y = j as i32 - k - 1;
                let top_mat_x = i as i32 - k - 1;
                if left_mat_y >= 0 {
                    ans[i][j] -= cum_mat[x][left_mat_y as usize];
                }
                if top_mat_x >= 0 {
                    ans[i][j] -= cum_mat[top_mat_x as usize][y];
                }

                if left_mat_y >= 0 && top_mat_x >= 0 {
                    ans[i][j] += cum_mat[top_mat_x as usize][left_mat_y as usize];
                }
            }
        }

        ans
    }
}
