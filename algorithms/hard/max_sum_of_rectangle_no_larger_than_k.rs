use std::collections::BTreeSet;
use std::ops::Bound::Included;

impl Solution {
    pub fn max_sum_submatrix(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        for i in 0..matrix.len() {
            for j in 1..matrix[i].len() {
                matrix[i][j] += matrix[i][j - 1];
            }
        }

        let mut ans = i32::MIN;
        for left in 0..matrix[0].len() {
            for right in left..matrix[0].len() {
                let mut set = BTreeSet::new();
                set.insert(0);
                let mut pre_sum = 0;
                for r in 0..matrix.len() {
                    pre_sum += matrix[r][right];
                    if left > 0 {
                        pre_sum -= matrix[r][left - 1];
                    }
                    if let Some(sum) = set.range(pre_sum - k..).next() {
                        ans = ans.max(pre_sum - sum);
                    }

                    set.insert(pre_sum);
                }
            }
        }
        ans
    }
}
