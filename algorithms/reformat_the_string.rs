impl Solution {
    pub fn reformat(s: String) -> String {
        let mut alpha = Vec::new();
        let mut digits = Vec::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                alpha.push(c);
            } else {
                digits.push(c);
            }
        }
        if (alpha.len() as i32 - digits.len() as i32).abs() > 1 {
            return "".to_string();
        }
        let mut ans = String::new();
        let mut long = alpha;
        let mut short = digits;
        if (long.len() < short.len()) {
            let t = long;
            long = short;
            short = t;
        }
        for i in 0..long.len() {
            ans.push(long[i]);
            if i < short.len() {
                ans.push(short[i]);
            }
        }
        ans
    }
}
