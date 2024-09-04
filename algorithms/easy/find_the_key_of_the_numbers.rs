fn get(nums: i32, d: i32) -> i32 {
    nums / d % 10
}

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut d = 1000;
        let mut ans = 0;
        while d > 0 {
            ans = ans * 10 + get(num1, d).min(get(num2, d)).min(get(num3, d));
            d /= 10;
        }
        ans
    }
}
