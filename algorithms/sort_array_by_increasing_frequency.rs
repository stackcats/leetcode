use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in &nums {
            *map.entry(*n).or_insert(0) += 1;
        }
        nums.sort_by(|a, b| {
            if map[a] == map[b] {
                b.cmp(a)
            } else {
                map[a].cmp(&map[b])
            }
        });
        nums
    }
}
