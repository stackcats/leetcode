impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        nums.dedup();
        
        let mut ans = 0;
        for i in 1..nums.len() - 1 {
            if nums[i] > nums[i-1] && nums[i] > nums[i+1] || nums[i] < nums[i-1] && nums[i] < nums[i+1] {
                ans += 1;
            }
        }
        
        ans
    }
}
