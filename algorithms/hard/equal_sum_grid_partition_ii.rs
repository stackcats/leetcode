use std::collections::HashMap;

fn can_remove(t: usize, b: usize, l: usize, r: usize, i: usize, j: usize) -> bool {
    if t == b {
        return j == r || j == l;
    }
    if l == r {
        return i == t || i == b;
    }

    true
}

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut mp = HashMap::new();

        let mut q = vec![vec![0; n]; m];
        q[0][0] = grid[0][0] as i64;

        mp.entry(q[0][0]).or_insert(vec![]).push((0, 0));

        for i in 1..n {
            q[0][i] = grid[0][i] as i64 + q[0][i - 1];
            mp.entry(grid[0][i] as i64).or_insert(vec![]).push((0, i));
        }

        for i in 1..m {
            q[i][0] = grid[i][0] as i64 + q[i - 1][0];
            mp.entry(grid[i][0] as i64).or_insert(vec![]).push((i, 0));
        }

        for i in 1..m {
            for j in 1..n {
                q[i][j] = grid[i][j] as i64 + q[i - 1][j] + q[i][j - 1] - q[i - 1][j - 1];
                mp.entry(grid[i][j] as i64).or_insert(vec![]).push((i, j));
            }
        }

        for i in 0..m - 1 {
            let diff = q[i][n - 1] * 2 - q[m - 1][n - 1];
            if diff == 0 {
                return true;
            }

            if diff < 0 {
                if let Some(v) = mp.get(&-diff)
                    && v.iter()
                        .any(|&(a, b)| a > i && can_remove(i + 1, m - 1, 0, n - 1, a, b))
                {
                    return true;
                }
            } else {
                if let Some(v) = mp.get(&diff)
                    && v.iter()
                        .any(|&(a, b)| a <= i && can_remove(0, i, 0, n - 1, a, b))
                {
                    return true;
                }
            }
        }

        for j in 0..n - 1 {
            let diff = q[m - 1][j] * 2 - q[m - 1][n - 1];
            if diff == 0 {
                return true;
            }
            if diff < 0 {
                if let Some(v) = mp.get(&-diff)
                    && v.iter()
                        .any(|&(a, b)| b > j && can_remove(0, m - 1, j + 1, n - 1, a, b))
                {
                    return true;
                }
            } else {
                if let Some(v) = mp.get(&diff)
                    && v.iter()
                        .any(|&(a, b)| b <= j && can_remove(0, m - 1, 0, j, a, b))
                {
                    return true;
                }
            }
        }

        false
    }
}
