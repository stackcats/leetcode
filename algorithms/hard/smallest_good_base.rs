impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n = n.parse::<u128>().unwrap();
        let max_len = n.ilog2() + 1;
        for len in (2..=max_len).rev() {
            let mut l = 2;
            let mut r = (n as f64).powf(1.0 / (len as f64 - 1.0)) as u128;
            while l <= r {
                let m = l + (r - l) / 2;
                let mut t = 0;
                for _ in 0..len {
                    t = t * m + 1;
                }
                if t == n {
                    return m.to_string();
                } else if t < n {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }

        (n - 1).to_string()
    }
}
