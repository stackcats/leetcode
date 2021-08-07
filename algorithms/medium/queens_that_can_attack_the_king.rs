use std::collections::HashSet;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set = HashSet::new();
        for q in &queens {
            set.insert(q);
        }
        let x = king[0];
        let y = king[1];
        let mut ans = Vec::new();
        // left
        for dy in (0..y).rev() {
            let v = vec![x, dy];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
        }

        // down
        for dx in (x + 1..8) {
            let v = vec![dx, y];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
        }

        // right
        for dy in (y + 1..8) {
            let v = vec![x, dy];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
        }

        // up
        for dx in (0..x).rev() {
            let v = vec![dx, y];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
        }

        // down left
        let mut d = 1;
        while x + d < 8 && y - d >= 0 {
            let v = vec![x + d, y - d];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
            d += 1;
        }

        // down right
        let mut d = 1;
        while x + d < 8 && y + d < 8 {
            let v = vec![x + d, y + d];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
            d += 1;
        }

        // up right
        let mut d = 1;
        while x - d >= 0 && y + d < 8 {
            let v = vec![x - d, y + d];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
            d += 1;
        }

        // up left
        let mut d = 1;
        while x - d >= 0 && y - d >= 0 {
            let v = vec![x - d, y - d];
            if set.contains(&v) {
                ans.push(v);
                break;
            }
            d += 1;
        }
        ans
    }
}
