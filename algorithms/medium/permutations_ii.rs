use std::collections::HashSet;

fn dfs(nums: &[i32]) -> HashSet<Vec<i32>> {
    if nums.len() == 1 {
        let mut set = HashSet::new();
        set.insert(vec![nums[0]]);
        return set;
    }

    let sets = dfs(&nums[1..]);
    let mut ans = HashSet::new();
    for s in sets.iter() {
        for i in 0..=s.len() {
            let mut t = s.clone();
            t.insert(i, nums[0]);
            ans.insert(t);
        }
    }
    ans
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        dfs(&nums).into_iter().collect()
    }
}
