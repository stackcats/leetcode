use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in &arr {
            *map.entry(*n).or_insert(0) += 1;
        }
        let mut vals = Vec::new();
        for v in map.values() {
            vals.push(v);
        }
        vals.sort_by(|a, b| b.cmp(a));
        let mut ans = 0;
        let mut sum = 0;
        for v in vals {
            if sum as usize >= arr.len() / 2 {
                break;
            }
            sum += v;
            ans += 1;
        }
        ans
    }
}
