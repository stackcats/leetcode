impl Solution {
    pub fn compare_bitonic_sums(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i - 1] as i64;
            } else {
                sum -= nums[i] as i64;
            }
        }

        match sum {
            0 => -1,
            n if n > 0 => 0,
            _ => 1,
        }
    }
}
