impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut l = String::new();
        let mut r = String::new();
        for c in s.chars() {
            if c == '1' {
                l.push(c);
            } else {
                r.push(c);
            }
        }
        if l.len() == 1 {
            format!("{}{}", r, l)
        } else {
            format!("{}{}{}", &l[..l.len() - 1], r, &l[l.len() - 1..])
        }
    }
}
