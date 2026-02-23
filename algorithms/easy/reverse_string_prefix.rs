impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let k = k as usize;
        let p = (&s[..k]).chars().rev().collect::<String>();
        let q = &s[k..].to_string();
        format!("{}{}", p, q)
    }
}
