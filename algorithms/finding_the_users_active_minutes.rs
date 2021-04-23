use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for log in logs {
            let id = log[0];
            let uam = log[1];
            let set = map.entry(id).or_insert(HashSet::new());
            set.insert(uam);
        }

        let mut ans = vec![0; k as usize];
        for set in map.values() {
            let ct = set.len();
            ans[ct - 1] += 1;
        }

        ans
    }
}
