fn gcd(mut a: i32, mut b: i32) -> i64 {
    while a % b != 0 {
        (a, b) = (b, a % b);
    }
    b as _
}

impl Solution {
    pub fn max_pair_strength(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let p = (nums[i] as i64) * (nums[j] as i64);
                let g = gcd(nums[i], nums[j]);
                ans = ans.max(p / (g * g));
            }
        }
        ans
    }
}
