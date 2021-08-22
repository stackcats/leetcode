fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut s = nums[0];
        let mut l = nums[0];
        for i in 1..nums.len() {
            s = s.min(nums[i]);
            l = l.max(nums[i]);
        }
        gcd(s, l)
    }
}
