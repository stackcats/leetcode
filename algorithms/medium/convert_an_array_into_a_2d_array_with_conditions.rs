use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut max_ct = 0;
        for n in nums {
            let ct = map.entry(n).or_insert(0);
            *ct += 1;
            max_ct = max_ct.max(*ct);
        }

        let mut ans = vec![vec![]; max_ct as usize];
        for (k, v) in map {
            for i in 0..v {
                ans[i].push(k);
            }
        }
        ans
    }
}
