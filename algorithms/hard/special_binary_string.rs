impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut ans = Vec::new();
        let mut ct = 0;
        let mut i = 0;
        for (j, c) in s.chars().enumerate() {
            ct += if c == '1' { 1 } else { -1 };

            if ct == 0 {
                let chunk = (&s[i + 1..j]).to_string();
                let t = format!("1{}0", Solution::make_largest_special(chunk));
                i = j + 1;
                ans.push(t);
            }
        }
        ans.sort_by(|a, b| b.cmp(a));
        ans.join("")
    }
}
