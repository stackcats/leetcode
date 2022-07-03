impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut inc = 1;
        let mut dec = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                dec = inc + 1;
            } else if nums[i] < nums[i-1] {
                inc = dec + 1;
            } 
        }
        inc.max(dec)
    }
}
