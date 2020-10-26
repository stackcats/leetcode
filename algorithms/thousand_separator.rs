impl Solution {
    pub fn thousand_separator(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        let mut i = 0;
        while n > 0 {
            s.push_str(&format!("{}", n % 10));
            n /= 10;
            i += 1;
            if i == 3 && n > 0 {
                s.push('.');
                i = 0;
            }
        }
        s.chars().rev().collect::<String>()
    }
}
