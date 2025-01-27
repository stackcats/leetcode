impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut ct = vec![0; 51];
        for &n in &nums {
            ct[n as usize] += 1;
        }

        let max_freq = (1..=50)
            .into_iter()
            .filter_map(|i| {
                if ct[i as usize] == 0 {
                    return None;
                }
                let mut cur = 0;
                let mut freq = 0;
                for &n in &nums {
                    if n == k {
                        cur -= 1;
                    }
                    if n == i {
                        cur += 1;
                    }
                    cur = cur.max(0);
                    freq = freq.max(cur);
                }
                Some(freq)
            })
            .max()
            .unwrap_or(0);

        ct[k as usize] + max_freq
    }
}
