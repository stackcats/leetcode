impl Solution {
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        for i in (0..nums.len() - 1).rev() {
            if nums[i] >= nums[i + + 1] {
                return i as i32 + 1;
            }
        }
        0
    }
}
