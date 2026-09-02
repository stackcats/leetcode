impl Solution {
    pub fn max_valid_pair_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        let mut t = 0;
        for i in 0..nums.len() - k {
            t = t.max(nums[i]);
            ans = ans.max(t + nums[i + k]);
        }
        ans
    }
}
