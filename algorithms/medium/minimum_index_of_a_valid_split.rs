use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut l = HashMap::new();
        let mut r = HashMap::new();
        for &n in &nums {
            *r.entry(n).or_insert(0) += 1;
        }

        let len = nums.len() as i32;
        for (i, n) in nums.iter().enumerate() {
            let i = i as i32;
            *l.entry(*n).or_insert(0) += 1;
            *r.entry(*n).or_insert(0) -= 1;
            if l.get(n).unwrap() * 2 > i + 1 && r.get(n).unwrap() * 2 > len - i - 1 {
                return i;
            }
        }

        -1
    }
}
