fn gcd(a: i32, b: i32) -> i32 {
    let g = a % b;
    if g == 0 { b } else { gcd(b, g) }
}

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut ma = nums[0];
        let mut v = Vec::new();
        for n in nums {
            ma = ma.max(n);
            v.push(gcd(ma, n));
        }
        v.sort();
        let mut l = 0;
        let mut r = v.len() - 1;
        let mut ans = 0i64;
        while l < r {
            ans += gcd(v[l], v[r]) as i64;
            l += 1;
            r -= 1;
        }
        ans
    }
}
