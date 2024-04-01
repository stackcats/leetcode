fn encrypt(mut n: i32) -> i32 {
    let mut m = 0;
    let mut digits = 0;
    while n > 0 {
        m = m.max(n % 10);
        n /= 10;
        digits += 1;
    }
    (0..digits).fold(0, |x, _| x * 10 + m)
}

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|n| encrypt(n)).sum()
    }
}
