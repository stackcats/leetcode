use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        for c in s.chars() {
            let d = map.entry(c).or_insert((c, 0));
            d.1 += 1;
        }

        let mut arr: Vec<&(char, i32)> = map.values().collect();
        arr.sort_by_key(|k| k.1);
        let mut ans = String::new();
        for i in (0..arr.len()).rev() {
            for j in 0..arr[i].1 {
                ans.push(arr[i].0);
            }
        }
        ans
    }
}
