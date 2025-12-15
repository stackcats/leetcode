impl Solution {
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        &nums[nums.len() - k as usize..].iter().sum::<i32>()
            - &nums[..k as usize].iter().sum::<i32>()
    }
}
