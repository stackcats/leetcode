// https://leetcode.com/problems/minimum-size-subarray-sum/

// sliding window
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut sum = 0;
        let mut ans = nums.len() + 1;
        for j in 0..nums.len() {
            sum += nums[j];
            while i <= j && sum >= s {
                ans = ans.min(j - i + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        if ans > nums.len() {
            return 0;
        }

        ans as i32
    }
}
