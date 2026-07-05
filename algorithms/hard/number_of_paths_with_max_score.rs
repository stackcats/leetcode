impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let m = board[0].len();
        let mut dp = vec![vec![vec![-1i64, 0]; m]; n];
        dp[n - 1][m - 1] = vec![0, 1];

        let g = board
            .into_iter()
            .map(|v| v.into_bytes())
            .collect::<Vec<_>>();

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if g[i][j] == b'S' || g[i][j] == b'X' {
                    continue;
                }

                for (x, y) in [(i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                    if x >= n || y >= m || dp[x][y][0] == -1 {
                        continue;
                    }
                    if dp[i][j][0] < dp[x][y][0] {
                        dp[i][j] = dp[x][y].clone();
                    } else if dp[i][j][0] == dp[x][y][0] {
                        dp[i][j][1] = (dp[i][j][1] + dp[x][y][1]) % 1000000007;
                    }
                }

                if dp[i][j][0] != -1 {
                    dp[i][j][0] += if g[i][j] == b'E' {
                        0
                    } else {
                        (g[i][j] - b'0') as i64
                    };
                }
            }
        }

        if dp[0][0][0] == -1 {
            vec![0, 0]
        } else {
            dp[0][0].iter().copied().map(|v| v as i32).collect()
        }
    }
}
