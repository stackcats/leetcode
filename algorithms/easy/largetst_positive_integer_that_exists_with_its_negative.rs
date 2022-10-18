use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut set = HashSet::new();

        for n in nums {
            set.insert(n);
            if set.contains(&-n) {
                ans = ans.max(n.abs());
            }
        }

        ans
    }
}
