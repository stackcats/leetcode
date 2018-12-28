// https://leetcode.com/problems/minimum-falling-path-sum/description/

fn min(a: &Vec<i32>) -> i32 {
    let mut m = a[0];
    for v in a {
        if m > *v {
            m = *v;
        }
    }
    m
}

impl Solution {
    pub fn min_falling_path_sum(a: &mut Vec<Vec<i32>>) -> i32 {
        let row = a.len();
        let col = a[0].len();
        if col == 1 {
            return a[0][0];
        }

        for i in 1..row {
            for j in 0..col {
                if j == 0 {
                    a[i][j] += min(&vec![a[i - 1][j], a[i - 1][j + 1]]);
                } else if j == col - 1 {
                    a[i][j] += min(&vec![a[i - 1][j], a[i - 1][j - 1]]);
                } else {
                    a[i][j] += min(&vec![a[i - 1][j], a[i - 1][j + 1], a[i - 1][j - 1]]);
                }
            }
        }

        min(&a[row - 1])
    }
}
