use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut mp = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            mp.entry(*n).or_insert(vec![]).push(i as i32);
        }

        let mut ans = Vec::new();
        let m = nums.len() as i32;

        for q in queries {
            let n = nums[q as usize];
            let v = &mp[&n];
            if v.len() == 1 {
                ans.push(-1);
                continue;
            }

            let i = v.binary_search(&q).unwrap();
            let lft = v[(i + v.len() - 1) % v.len()];
            let rht = v[(i + 1) % v.len()];
            let a = (lft - q).abs();
            let b = (rht - q).abs();
            let c = m - a;
            let d = m - b;

            ans.push(a.min(b).min(c).min(d));
        }

        ans
    }
}
