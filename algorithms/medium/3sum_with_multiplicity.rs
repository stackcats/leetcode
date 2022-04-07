impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0i64;
        let mut ct = vec![0; 101];
        for &n in &arr {
            ct[n as usize] += 1;
        }

        for a in 0..=100 {
            for b in 0..=100 {
                let c = target - a - b;
                if c < 0 || c > 100 {
                    continue;
                }

                let a = a as usize;
                let b = b as usize;
                let c = c as usize;
                
                if a == b && a == c {
                    ans += ct[a] * (ct[a] - 1) * (ct[a] - 2) / 6;
                } else if a == b && a != c {
                    ans += ct[a] * (ct[a] - 1) / 2 * ct[c]
                } else if a < b && b < c {
                    ans += ct[a] * ct[b] * ct[c];
                }
                ans %= 1000000007;
            }
        }

        ans as i32
    }
}
