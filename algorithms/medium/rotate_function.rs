impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut f = nums
            .iter()
            .zip(0..nums.len())
            .map(|(n, i)| n * (i as i32))
            .sum::<i32>();
        let mut ans = f;
        let n = nums.len();
        for k in 1..n {
            f = f + sum - (n as i32) * nums[n - k];
            ans = ans.max(f);
        }
        ans
    }
}
