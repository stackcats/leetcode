impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut ct = vec![0; 26];
        for b in s.as_bytes() {
            ct[(b - b'a') as usize] += 1;
        }

        let mut even = i32::MAX;
        let mut odd = 0;
        for v in ct {
            if v == 0 {
                continue;
            }

            if v % 2 == 0 {
                even = even.min(v);
            } else {
                odd = odd.max(v);
            }
        }

        odd - even
    }
}
