use std::collections::{HashSet, VecDeque};

fn is_valid(s: &str) -> bool {
    let mut ct = 0;
    for c in s.chars() {
        if c == '(' {
            ct += 1;
        } else if c == ')' {
            if ct == 0 {
                return false;
            }
            ct -= 1;
        }
    }
    ct == 0
}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut q = VecDeque::new();
        q.push_back(s);
        let mut seen = HashSet::new();
        let mut res: Vec<String> = Vec::new();

        while let Some(s) = q.pop_front() {
            if seen.contains(&s) {
                continue;
            }

            seen.insert(s.to_string());

            if is_valid(&s) {
                if res.is_empty() || s.len() == res[0].len() {
                    res.push(s.to_string());
                    continue;
                }
                break;
            }

            for i in 0..s.len() {
                let mut t = s.to_string();
                let c = t.remove(i);
                if c == '(' || c == ')' {
                    q.push_back(t);
                }
            }
        }

        res
    }
}
