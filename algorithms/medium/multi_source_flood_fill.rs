use std::collections::VecDeque;

impl Solution {
    pub fn color_grid(n: i32, m: i32, mut sources: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut g = vec![vec![0; m as usize]; n as usize];
        let mut q = VecDeque::new();

        sources.sort_by_key(|s| -s[2]);

        for s in &sources {
            let (r, c, color) = (s[0], s[1], s[2]);
            g[r as usize][c as usize] = color;
            q.push_back((color, r, c));
        }

        while let Some((color, r, c)) = q.pop_front() {
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let (r, c) = (r + dr, c + dc);
                if r >= 0 && r < n && c >= 0 && c < m && g[r as usize][c as usize] == 0 {
                    g[r as usize][c as usize] = color;
                    q.push_back((color, r, c));
                }
            }
        }

        g
    }
}
