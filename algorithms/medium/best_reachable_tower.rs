fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

impl Solution {
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        let mut q = -1;
        for t in towers {
            if manhattan(t[0], t[1], center[0], center[1]) <= radius {
                if t[2] > q || t[2] == q && (t[0] < ans[0] || (t[0] == ans[0] && t[1] < ans[1])) {
                    ans = vec![t[0], t[1]];
                    q = t[2];
                }
            }
        }

        ans
    }
}
