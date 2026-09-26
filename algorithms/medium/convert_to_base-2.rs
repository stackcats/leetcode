impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let base = -2;

        let mut ans = String::new();
        while n != 0 {
            let mut r = n % base;
            n /= base;
            if r < 0 {
                r += -1 * base;
                n += 1;
            }
            ans = format!("{}{}", r, ans);
        }

        ans
    }
}
