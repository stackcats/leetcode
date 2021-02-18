impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut r = String::new();
        let mut is_rev = true;
        for i in (0..s.len()).step_by(k as usize) {
            let j = (i + k as usize).min(s.len());
            if is_rev {
                r.push_str(&s[i..j].iter().rev().collect::<String>());
            } else {
                r.push_str(&s[i..j].iter().collect::<String>());
            }

            is_rev = !is_rev;
        }
        r
    }
}
