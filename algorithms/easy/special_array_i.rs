impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                return false;
            }
        }
        true
    }
}
