impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ct = 0;
        let mut ans = Vec::new();
        for c in s.chars() {
            if ct == 0 {
                ct += 1;
                continue;
            }
            if c == '(' {
                ct += 1;
                ans.push(c);
            } else {
                ct -= 1;
                if ct == 0 {
                    continue;
                }
                ans.push(c);
            }
        }
        ans.iter().collect::<String>()
    }
}
