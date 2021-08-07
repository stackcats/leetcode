impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut r = m;
        let mut c = n;
        for op in &ops {
            r = r.min(op[0]);
            c = c.min(op[1]);
        }
        r * c
    }
}
