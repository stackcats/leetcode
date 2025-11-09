use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut ans = usize::MAX;
        let mut mp = HashMap::new();
        for (k, n) in nums.into_iter().enumerate() {
            let v = mp.entry(n).or_insert(vec![]);
            if v.len() > 1 {
                for i in 0..v.len() {
                    for j in i + 1..v.len() {
                        let d = v[j] - v[i] + k - v[i] + k - v[j];
                        ans = ans.min(d);
                    }
                }
            }
            v.push(k);
        }
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
