// https://leetcode.com/problems/unique-paths/description/

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut d = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 && j == 0 {
                    d[i][j] = 1;
                } else if i == 0 {
                    d[i][j] = d[i][j - 1];
                } else if j == 0 {
                    d[i][j] = d[i - 1][j];
                } else {
                    d[i][j] = d[i][j - 1] + d[i - 1][j];
                }
            }
        }

        d[m as usize - 1][n as usize - 1]
    }
}
