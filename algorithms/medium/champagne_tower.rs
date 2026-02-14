impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut g = vec![vec![0.0; 102]; 102];
        g[0][0] = poured as f64;
        let query_glass = query_glass as usize;
        for i in 0..100 {
            for j in 0..=query_glass {
                let half = (g[i][j] - 1.0).max(0.0) / 2.0;
                g[i + 1][j] += half;
                g[i + 1][j + 1] += half;
            }
        }
        g[query_row as usize][query_glass as usize].min(1.0)
    }
}
