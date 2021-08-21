impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::new();
        let mut last_char = s.chars().nth(0).unwrap();
        ans.push(last_char);
        let mut ct = 1;
        for c in s.chars().skip(1) {
            if c == last_char {
                ct += 1;
                if ct > 2 {
                    continue;
                }
            } else {
                ct = 1;
            }

            ans.push(c);
            last_char = c;
        }
        ans
    }
}
