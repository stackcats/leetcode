// https://leetcode.com/problems/strange-printer/description/

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let bs = s.as_bytes();

        let mut d = vec![vec![0; n]; n];

        for i in 0..n {
            d[i][i] = 1;
        }

        for dis in 1..n {
            for i in 0..(n - dis) {
                let j = i + dis;
                d[i][j] = (j - i) as i32 + 1;
                for k in i..j {
                    let mut total = d[i][k] + d[k + 1][j];
                    if bs[j] == bs[k] {
                        total -= 1;
                    }
                    d[i][j] = d[i][j].min(total);
                }
            }
        }

        d[0][n - 1]
    }
}
