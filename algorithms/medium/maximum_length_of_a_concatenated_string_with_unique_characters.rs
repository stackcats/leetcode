impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp = vec![0];
        let mut ans = 0;
        for s in arr {
            let mut bit = 0u32;
            let size = s.len();
            for c in s.into_bytes() {
                bit |= 1 << (c - b'a');
            }

            if (bit.count_ones() as usize) != size {
                continue;
            }

            let mut brr = Vec::new();
            for b in &dp {
                let c = bit & b;
                if c.count_ones() > 0 {
                    continue;
                }
                let c = bit | b;
                ans = ans.max(c.count_ones());
                brr.push(c);
            }
            dp.append(&mut brr);
        }
        ans as _
    }
}
