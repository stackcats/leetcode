impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut current = nums[0];
        for i in 1..nums.len() {
            if nums[i] <= current {
                ans += current + 1 - nums[i];
                current += 1;
            } else {
                current = nums[i];
            }
        }
        ans
    }
}
