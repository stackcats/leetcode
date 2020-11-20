fn can_rotated(mut n: i32) -> bool {
    let mut ans = false;
    while n > 0 {
        let d = n % 10;
        if d == 3 || d == 4 || d == 7 {
            return false;
        }
        if d == 2 || d == 5 || d == 6 || d == 9 {
            ans = true;
        }
        n /= 10;
    }
    ans
}

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if can_rotated(i) {
                ans += 1;
            }
        }
        ans
    }
}
