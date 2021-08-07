use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        for i in 0..group_sizes.len() {
            map.entry(group_sizes[i] as usize)
                .or_insert(vec![])
                .push(i as i32);
        }
        let mut ans = Vec::new();
        let mut sub_arr = Vec::new();
        for k in map.keys() {
            let mut i = 0;
            for n in &map[k] {
                sub_arr.push(*n);
                i += 1;
                if i == *k {
                    ans.push(sub_arr);
                    sub_arr = Vec::new();
                    i = 0;
                }
            }
        }
        ans
    }
}
