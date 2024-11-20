impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut right: i32 = nums.iter().sum();
        let mut left = 0;
        let mut ct = 0;
        for n in nums {
            if n == 0 {
                ct += match (left - right).abs() {
                    0 => 2,
                    1 => 1,
                    _ => 0,
                };
            } else {
                left += n;
                right -= n;
            }
        }
        ct 
    }
}
