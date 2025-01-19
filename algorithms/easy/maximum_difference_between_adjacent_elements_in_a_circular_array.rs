impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut ans = (nums.last().unwrap() - nums[0]).abs();
        for i in 1..nums.len() {
            ans = (nums[i] - nums[i - 1]).abs().max(ans);
        }
        ans
    }
}
