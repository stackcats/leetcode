use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        let mut st = HashSet::new();
        for (i, n) in nums.into_iter().enumerate() {
            if st.contains(&n) {
                continue;
            }
            if let Some(j) = mp.get(&n) {
                if *j == i - 1 {
                    mp.insert(n, i);
                } else {
                    mp.remove(&n);
                    st.insert(n);
                }
            } else {
                mp.insert(n, i);
            }
        }
        mp.len() as _
    }
}
