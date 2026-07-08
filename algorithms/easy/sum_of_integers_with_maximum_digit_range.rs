fn range(mut n: i32) -> i32 {
    let mut l = i32::MIN;
    let mut s = i32::MAX;
    while n > 0 {
        let d = n % 10;
        l = l.max(d);
        s = s.min(d);
        n /= 10;
    }
    l - s
}

impl Solution {
    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max_range = 0;
        for n in nums {
            let r = range(n);
            if max_range < r {
                max_range = r;
                ans = n;
            } else if max_range == r {
                ans += n;
            }
        }
        ans
    }
}
