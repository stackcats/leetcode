impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        if n == 0 {
            return 0;
        }

        let mut v = Vec::new();
        while n > 0 {
            if n % 10 != 0 {
                v.insert(0, n % 10);
            }
            n /= 10;
        }

        let mut sum = 0;
        let mut x = 0;
        for m in v {
            x = x * 10 + m as i64;
            sum += m as i64;
        }

        sum * x
    }
}
