impl Solution {
    pub fn max_sum(mut nums: Vec<i32>, k: i32, mul: i32) -> i64 {
        nums.sort_by(|a, b| b.cmp(&a));
        let mut ans = 0;
        let mut mul = mul as i64;
        for i in 0..(k as usize) {
            if mul == 0 {
                ans += nums[i] as i64;
            } else {
                ans += (nums[i] as i64) * mul;
                mul -= 1;
            }
        }
        ans
    }
}
