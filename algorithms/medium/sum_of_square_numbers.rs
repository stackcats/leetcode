impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut a: i64 = 0;
        let mut b: i64 = (c as f64).sqrt().round() as i64;

        while a <= b {
            let n = a * a + b * b;
            if n == c {
                return true;
            }
            if n < c {
                a += 1;
            } else {
                b -= 1;
            }
        }

        false
    }
}
