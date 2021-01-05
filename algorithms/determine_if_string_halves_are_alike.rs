use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut set = HashSet::new();
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        for c in &vowels {
            set.insert(*c);
        }

        let mut ct = 0;
        let mut i = 0;
        let pivot = s.len() / 2;
        for c in s.chars() {
            if set.contains(&c) {
                if i < pivot {
                    ct += 1;
                } else {
                    ct -= 1;
                }
            }
            i += 1;
        }
        ct == 0
    }
}
