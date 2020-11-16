impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let all: i32 = nums.iter().sum();
        let mut left = 0;
        for i in 0..nums.len() {
            let right = all - left - nums[i];
            if left == right {
                return i as i32;
            }
            left += nums[i];
        }
        -1
    }
}
