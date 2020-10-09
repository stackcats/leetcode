use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut ans = HashSet::with_capacity(emails.len());
        for email in &emails {
            let vec = email.chars().collect::<Vec<_>>();
            let mut s = String::with_capacity(email.len());
            let mut i = 0;
            while vec[i] != '@' {
                if vec[i] == '.' {
                    i += 1;
                    continue;
                }
                if vec[i] == '+' {
                    break;
                }
                s.push(vec[i]);
                i += 1;
            }
            while vec[i] != '@' {
                i += 1;
            }
            while i < vec.len() {
                s.push(vec[i]);
                i += 1;
            }

            ans.insert(s);
        }
        ans.len() as i32
    }
}
