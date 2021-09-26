use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        for &n in &nums {
            let mut new_map = HashMap::new();
            for (k, v) in map.iter() {
                *new_map.entry(k + n).or_insert(0) += v;
                *new_map.entry(k - n).or_insert(0) += v;
            }
            map = new_map;
        }

        *map.get(&target).take().unwrap_or(&0)
    }
}
