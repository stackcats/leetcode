use std::collections::LinkedList;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut lst = LinkedList::new();
        for c in s.chars() {
            if lst.len() == 0 {
                lst.push_back(c);
            } else if *lst.back().unwrap() == c {
                lst.pop_back();
            } else {
                lst.push_back(c);
            }
        }
        lst.iter().collect::<String>()
    }
}
