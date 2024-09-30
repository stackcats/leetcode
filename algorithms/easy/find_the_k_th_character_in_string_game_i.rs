fn aux(s: &str) -> String {
    let mut t = String::new();
    for b in s.bytes() {
        let c = (b'a' + (b - b'a' + 1) % 26) as char;
        t.push(c);
    }
    t
}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut s = "a".to_string();
        while s.len() < k as usize {
            s = format!("{}{}", s, aux(&s));
        }
        s.chars().nth(k as usize - 1).unwrap()
    }
}
