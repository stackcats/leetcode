impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut sum = nums.iter().sum::<i32>();
        let mut ans = 0;
        let mut size = nums.len() - 1;
        for (i, &n) in nums.iter().enumerate() {
            sum -= n;
            let avg = (sum as f64) / (size as f64);
            size -= 1;
            if (n as f64) > avg {
                ans += 1;
            }
        }
        ans
    }
}
