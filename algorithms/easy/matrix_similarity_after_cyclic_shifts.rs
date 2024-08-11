impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize;
        let n = mat[0].len();
        for row in &mat {
            if (0..n).any(|i| row[i] != row[(i + k) % n]) {
                return false;
            }
        }
        true
    }
}
