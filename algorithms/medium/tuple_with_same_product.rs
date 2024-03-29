use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                *map.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }

        map.into_iter().fold(0, |acc, (_, v)| acc + v * (v - 1) * 4) 
    }
}
