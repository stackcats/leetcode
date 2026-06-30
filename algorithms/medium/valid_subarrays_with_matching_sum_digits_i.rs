fn check(mut sum: i64, x: i64) -> bool {
    if sum % 10 != x {
        return false;
    }

    while sum >= 10 {
        sum /= 10;
    }

    sum == x
}

impl Solution {
    pub fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i32 {
        let mut pre = vec![0; nums.len() + 1];
        for (i, n) in nums.into_iter().enumerate() {
            pre[i + 1] = pre[i] + n as i64;
        }
        let mut ans = 0;
        for i in 0..pre.len() {
            for j in i + 1..pre.len() {
                let sum = pre[j] - pre[i];
                if check(sum, x as i64) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
