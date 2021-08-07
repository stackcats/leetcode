use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut arr = nums.clone();
        arr.sort_by(|a, b| b.cmp(a));
        let mut map = HashMap::new();
        for i in 0..arr.len() {
            map.insert(arr[i], i + 1);
        }
        let mut ans = Vec::new();
        for n in &nums {
            let pos = match map[n] {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                n => format!("{}", n),
            };
            ans.push(pos);
        }
        ans
    }
}
