use std::f64::consts::PI;

fn loc(a: f64, b: f64, c: f64) -> f64 {
    ((b * b + c * c - a * a) / (2. * b * c)).acos() * 180. / PI
}

impl Solution {
    pub fn internal_angles(mut sides: Vec<i32>) -> Vec<f64> {
        sides.sort();
        let (a, b, c) = (sides[0] as f64, sides[1] as f64, sides[2] as f64);
        if a + b <= c || a + c <= b || b + c <= a {
            return vec![];
        }

        let mut v = vec![loc(a, b, c), loc(c, b, a), loc(b, c, a)];
        v.sort_by(f64::total_cmp);
        v
    }
}
