impl Solution {
    pub fn maximum_median_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut i = nums.len() - 2;
        for _ in 0..(nums.len() / 3) {
            ans += nums[i] as i64;
            i -= 2;
        }
        ans
    }
}
