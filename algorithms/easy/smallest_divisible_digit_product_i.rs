fn digit_product(mut n: i32) -> i32 {
    let mut m = 1;

    while n > 0 {
        m *= n % 10;
        n /= 10;
    }

    m
}

impl Solution {
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        while digit_product(n) % t != 0 {
            n += 1;
        }
        n
    }
}
