impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();
        for (i, n) in nums.iter().enumerate() {
            right_sum -= n;
            if left_sum == right_sum {
                return i as i32;
            }
            left_sum += n;
        }
        -1
    }
}
