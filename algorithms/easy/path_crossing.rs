use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut set = HashSet::new();
        let mut i = 0;
        let mut j = 0;
        set.insert(format!("{} {}", i, j));
        for d in path.chars() {
            if d == 'N' {
                i += 1;
            } else if d == 'S' {
                i -= 1;
            } else if d == 'E' {
                j += 1;
            } else {
                j -= 1;
            }
            let s = format!("{} {}", i, j);
            if set.contains(&s) {
                return true;
            }
            set.insert(s);
        }
        false
    }
}
