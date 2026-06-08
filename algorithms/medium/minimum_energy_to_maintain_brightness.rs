fn h(v: Vec<i32>, ct: i64) -> i64 {
    let len = v[1] - v[0] + 1;
    len as i64 * ct
}

impl Solution {
    pub fn min_energy(n: i32, brightness: i32, mut intervals: Vec<Vec<i32>>) -> i64 {
        let ct = (brightness as f64 / 3.0).ceil() as i64;
        let mut ans = 0;
        let mut pre = vec![];
        intervals.sort_by_key(|v| v[0]);
        for v in intervals {
            if pre.is_empty() {
                pre = v;
                continue;
            }
            let (a, b) = (pre[0], pre[1]);
            let (c, d) = (v[0], v[1]);
            if b < c {
                ans += h(pre, ct);
                pre = v;
            } else {
                pre = vec![a, b.max(d)];
            }
        }

        ans + h(pre, ct)
    }
}
