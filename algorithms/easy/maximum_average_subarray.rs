impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut ans = 0;
        let mut sum = nums.iter().take(k).sum();
        ans = sum;
        for i in k..nums.len() {
            sum += nums[i];
            sum -= nums[i - k];
            ans = ans.max(sum);
        }
        ans as f64 / k as f64
    }
}
