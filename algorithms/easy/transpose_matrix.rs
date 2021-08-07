// https://leetcode.com/problems/transpose-matrix/

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut b = vec![vec![0; a.len()]; a[0].len()];
        for i in 0..a.len() {
            for j in 0..a[i].len() {
                b[j][i] = a[i][j];
            }
        }

        b
    }
}
