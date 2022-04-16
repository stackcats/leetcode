#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a % b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    b
}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        for d in 2..=n {
            for i in 1..d {
                if gcd(i, d) == 1 {
                    ans.push(format!("{}/{}", i, d));
                }
            }
        }

        ans
    }
}
