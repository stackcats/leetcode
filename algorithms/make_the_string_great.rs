use std::collections::LinkedList;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut is_good = false;
        let mut lst = LinkedList::new();
        for c in s.chars() {
            if lst.len() == 0 {
                lst.push_back(c);
                continue;
            }
            if let Some(b) = lst.back() {
                if b.to_ascii_uppercase() == c.to_ascii_uppercase() && *b != c {
                    lst.pop_back();
                } else {
                    lst.push_back(c);
                }
            }
        }
        lst.iter().collect::<String>()
    }
}
