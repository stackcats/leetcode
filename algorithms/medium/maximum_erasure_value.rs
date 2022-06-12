use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        let mut left = 0;
        let mut curr = 0;
        for (right, n) in nums.iter().enumerate() {
            if let Some(&pre) = map.get(n) {  
                ans = ans.max(curr);
                while left <= pre {
                    curr -= nums[left];
                    map.remove(&nums[left]);
                    left += 1;
                }
            } 
            map.insert(*n, right);
            curr += *n;
        }
        
        ans.max(curr)
    }
}
