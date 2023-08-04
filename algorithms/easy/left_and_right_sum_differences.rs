impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut right = nums.iter().sum::<i32>();
        let mut left = 0;
        let mut res = vec![0; nums.len()];
        for (i, n) in nums.into_iter().enumerate() {
            res[i] = (right - left - n).abs();
            left += n;
            right -= n;
        }
        res
    }
}
