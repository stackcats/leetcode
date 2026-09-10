const DIRS: [(i32, i32); 8] = [
    (-2, 1),
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
];

fn dfs(n: i32, k: i32, r: i32, c: i32, dp: &mut Vec<Vec<Vec<Option<f64>>>>) -> f64 {
    if k == 0 {
        return 1.0;
    }

    if let Some(v) = dp[r as usize][c as usize][k as usize] {
        return v;
    }

    let mut p = 0.0;
    for &(dr, dc) in &DIRS {
        let (nr, nc) = (r + dr, c + dc);
        if nr >= 0 && nr < n && nc >= 0 && nc < n {
            p += 0.125 * dfs(n, k - 1, nr, nc, dp);
        }
    }

    dp[r as usize][c as usize][k as usize] = Some(p);
    p
}

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![vec![None; k as usize + 1]; n as usize]; n as usize];
        dfs(n, k, row, column, &mut dp)
    }
}
