fn k(x: &[i32], y: &[i32]) -> Option<f64> {
    if y[0] == x[0] {
        return None;
    }
    Some((y[1] - x[1]) as f64 / (y[0] - x[0]) as f64)
}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut s = k(&coordinates[0], &coordinates[1]);
        for i in 2..coordinates.len() {
            if s != k(&coordinates[i], &coordinates[i - 1]) {
                return false;
            }
        }
        true
    }
}
