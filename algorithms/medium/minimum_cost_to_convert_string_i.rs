impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut g = vec![vec![i64::MAX; 26]; 26];
        for i in 0..original.len() {
            let s = (original[i] as u8 - b'a') as usize;
            let t = (changed[i] as u8 - b'a') as usize;
            g[s][t] = g[s][t].min(cost[i] as i64);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if g[i][k] != i64::MAX && g[k][j] != i64::MAX {
                        g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                    }
                }
            }
        }

        let mut ans = 0;

        let source = source.as_bytes();
        let target = target.as_bytes();
        for i in 0..source.len() {
            let s = (source[i] as u8 - b'a') as usize;
            let t = (target[i] as u8 - b'a') as usize;
            if s == t {
                continue;
            }

            if g[s][t] == i64::MAX {
                return -1;
            }

            ans += g[s][t];
        }
        ans
    }
}
