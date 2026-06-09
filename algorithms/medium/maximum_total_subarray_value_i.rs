impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut ma = i32::MIN;
        let mut mi = i32::MAX;
        let mut i = 0;
        let mut j = 0;
        for k in 0..nums.len() {
            if ma < nums[k] {
                i = k;
                ma = nums[k];
            }
            if mi > nums[k] {
                j = k;
                mi = nums[k];
            }
        }
        (ma - mi) as i64 * (k as i64)
    }
}
