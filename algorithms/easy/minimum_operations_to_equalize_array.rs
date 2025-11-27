impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                return 1;
            }
        }
        0
    }
}
