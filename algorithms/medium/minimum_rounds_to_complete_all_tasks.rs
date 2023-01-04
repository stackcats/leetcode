use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        for t in tasks {
            *mp.entry(t).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (_, v) in mp {
            if v == 1 {
                return -1;
            } else {
                ans += (v as f64 / 3.0).ceil() as i32;
            }
        }
        ans
    }
}
