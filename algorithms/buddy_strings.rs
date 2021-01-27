use std::collections::HashMap;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() < 2 {
            return false;
        }
        if a.len() != b.len() {
            return false;
        }
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let mut a1 = a[0];
        let mut a2 = a[0];
        let mut b1 = b[0];
        let mut b2 = b[0];
        let mut diff = 0;
        let mut map = HashMap::new();
        for i in 0..a.len() {
            *map.entry(a[i]).or_insert(0) += 1;
            if a[i] != b[i] {
                if diff == 0 {
                    diff += 1;
                    a1 = a[i];
                    b1 = b[i];
                } else if diff == 1 {
                    diff += 1;
                    a2 = a[i];
                    b2 = b[i];
                } else {
                    return false;
                }
            }
        }
        if diff == 1 {
            return false;
        }

        if diff == 2 {
            return a1 == b2 && a2 == b1;
        }

        for v in map.values() {
            if *v >= 2 {
                return true;
            }
        }
        false
    }
}
