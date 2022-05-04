use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for n in nums {
            let ct = map.entry(k - n).or_insert(0);
            if *ct == 0 {
                *map.entry(n).or_insert(0) += 1;
            } else {
                *ct -= 1;
                ans += 1;
            }
            
        }
        
        ans
    }
}
