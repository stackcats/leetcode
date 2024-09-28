use std::collections::HashSet;

fn sort(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut st = HashSet::new();
        let mut ans = HashSet::new();
        for n in nums {
            if st.contains(&(n - k)) {
                ans.insert(sort(n - k, n));
            }
            if st.contains(&(k + n)) {
                ans.insert(sort(n + k, n));
            }
            st.insert(n);
        }
        ans.len() as i32
    }
}
