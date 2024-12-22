impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        for i in 0..nums.len() - 2 {
            if (nums[i] + nums[i + 2]) * 2 == nums[i + 1] {
                ct += 1;
            }
        }
        ct
    }
}
