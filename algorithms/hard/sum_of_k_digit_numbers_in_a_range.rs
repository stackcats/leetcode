fn modpow(mut n: i64, mut exp: u32, m: i64) -> i64 {
    let mut ans = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            ans = ans * n % m;
        }
        n = n * n % m;
        exp >>= 1;
    }
    ans
}

impl Solution {
    pub fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
        let m = 10i64.pow(9) + 7;
        let (l, r, k) = (l as i64, r as i64, k as u32);
        let ct = r - l + 1;
        let a = (l + r) * ct / 2;
        let b = modpow(ct, k - 1, m);
        let c = (modpow(10, k, m) - 1) * modpow(9, m as u32 - 2, m) % m;
        [a, b, c].iter().fold(1, |acc, &n| acc * n % m) as _
    }
}
