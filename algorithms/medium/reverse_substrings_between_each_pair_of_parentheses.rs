impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ans = String::new();
        for c in s.chars() {
            if c != ')' {
                ans.push(c);
                continue;
            }
            let mut tmp = String::new();
            while let Some(x) = ans.pop() {
                if x == '(' {
                    ans.push_str(&tmp);
                    break;
                }
                tmp.push(x);
            }
        }
        ans
    }
}
