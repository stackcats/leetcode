impl Solution {
    pub fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j] == 1 {
                    ans[j] += 1;
                }
            }
        }
        ans
    }
}
