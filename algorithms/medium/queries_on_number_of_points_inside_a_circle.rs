impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        for q in &queries {
            let mut ct = 0;
            for p in &points {
                let dx = p[0] - q[0];
                let dy = p[1] - q[1];
                if dx * dx + dy * dy <= q[2] * q[2] {
                    ct += 1;
                }
            }
            ans.push(ct);
        }
        ans
    }
}
