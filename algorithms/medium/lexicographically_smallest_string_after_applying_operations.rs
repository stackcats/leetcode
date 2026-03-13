use std::collections::{HashSet, VecDeque};

fn add(s: &str, a: u8) -> String {
    let mut t = String::new();
    let s = s.as_bytes();
    for i in 0..s.len() {
        if i % 2 == 0 {
            t.push(s[i] as char);
        } else {
            let mut d = s[i] - b'0' + a;
            d %= 10;
            t.push((b'0' + d) as char);
        }
    }
    t
}

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let a = a as u8;
        let b = s.len() - b as usize;
        let mut q = VecDeque::new();
        let mut ans = s.to_string();
        q.push_back(s);
        let mut st = HashSet::new();
        while let Some(s) = q.pop_front() {
            if st.contains(&s) {
                continue;
            }

            ans = ans.min(s.clone());

            st.insert(s.to_string());

            let mut t = add(&s, a);
            while t != s {
                q.push_back(t.to_string());
                t = add(&t, a);
            }

            q.push_back(format!("{}{}", &s[b..], &s[..b]));
        }

        ans
    }
}
