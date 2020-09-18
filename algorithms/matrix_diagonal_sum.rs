impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = mat.len() - 1;
        for k in 0..mat.len() {
            ans += if i == j { mat[i][k] } else { mat[i][k] + mat[j][k] };
            i += 1;
            j -= 1;
        }
        ans
    }
}
