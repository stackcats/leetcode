impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut j = nums.len() - 1;
        while j > 0 {
            for i in 0..j {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            j -= 1;
        }
        nums[j]
    }
}
