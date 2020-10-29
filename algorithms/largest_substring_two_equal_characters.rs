use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut ans = -1;
        let mut map = HashMap::new();
        let arr = s.chars().collect::<Vec<char>>();
        for i in 0..arr.len() {
            if map.contains_key(&arr[i]) {
                if ans < i as i32 - map[&arr[i]] - 1 {
                    ans = i as i32 - map[&arr[i]] - 1;
                }
            } else {
                map.insert(arr[i], i as i32);
            }
        }

        ans
    }
}
