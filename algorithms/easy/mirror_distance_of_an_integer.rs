fn reverse(mut n: i32) -> i32 {
    let mut m = 0;
    while n > 0 {
        m = m * 10 + n % 10;
        n /= 10;
    }
    m
}

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (reverse(n) - n).abs()
    }
}
