impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut sum = res;
        for n in &nums {
            sum += *n;
            if sum < 1 {
                res += (1 - sum);
                sum = 1;
            }
        }
        res
    }
}
