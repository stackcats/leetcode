impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut ans = None;
        let mut ct = vec![0; 101];
        let mut ma = i32::MIN;
        for n in nums {
            ma = ma.max(n);
            if n <= 0 {
                continue;
            }
            if ct[n as usize] == 0 {
                ans = ans.map_or(Some(n), |v| Some(v + n));
                ct[n as usize] = 1;
            }
        }

        ans.unwrap_or(ma)
    }
}
