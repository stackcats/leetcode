impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut p = 1;
        for n in nums {
            ans += n * p;
            p *= -1;
        }
        ans
    }
}
