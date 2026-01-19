impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut pre_sums = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                pre_sums[i][j] += pre_sums[i][j - 1] + pre_sums[i - 1][j] + mat[i - 1][j - 1]
                    - pre_sums[i - 1][j - 1];
            }
        }

        let mut ans = 0;
        for i in 0..=m {
            for j in 0..=n {
                // only need to check if there is a `side` > `ans`
                let mut side = ans + 1;
                while i + side <= m && j + side <= n {
                    let sum = pre_sums[i + side][j + side] + pre_sums[i][j]
                        - pre_sums[i][j + side]
                        - pre_sums[i + side][j];

                    if sum > threshold {
                        break;
                    }
                    ans = ans.max(side);
                    side += 1;
                }
            }
        }

        ans as _
    }
}
