impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        (nums[0] * nums[1] * nums[nums.len() - 1])
            .max(nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3])
    }
}
