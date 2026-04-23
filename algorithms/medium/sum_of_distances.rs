use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut mp = HashMap::new();
        for i in 0..nums.len() {
            mp.entry(nums[i]).or_insert(vec![]).push(i as i64);
        }

        let mut ans = vec![0; nums.len()];

        for v in mp.values() {
            let mut rht: i64 = v.iter().sum();
            let mut lft = 0;
            let n = v.len() - 1;
            for i in 0..v.len() {
                rht -= v[i];
                ans[v[i] as usize] = rht - (n - i - i) as i64 * v[i] - lft;
                lft += v[i];
            }
        }

        ans
    }
}
