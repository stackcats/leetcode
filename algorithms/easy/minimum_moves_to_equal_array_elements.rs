impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = nums[0];
        for i in 0..nums.len() {
            sum += nums[i];
            if min > nums[i] {
                min = nums[i];
            }
        }
        sum - (nums.len() as i32) * min
    }
}
