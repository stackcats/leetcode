use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut pre_sum = 0;
        let mut ans = 0;
        map.insert(0, 1);
        for n in nums {
            pre_sum += n;
            if let Some(ct) = map.get(&(pre_sum - k)) {
                ans += *ct;
            }
            *map.entry(pre_sum).or_insert(0) += 1;
        }
        ans
    }
}
