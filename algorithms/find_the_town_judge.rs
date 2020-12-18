use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.len() == 0 {
            return 1;
        }
        let mut map = HashMap::new();
        for each in &trust {
            *map.entry(each[1]).or_insert(0) += 1;
            *map.entry(each[0]).or_insert(0) -= 1;
        }
        for k in map.keys() {
            if map[k] == n - 1 {
                return *k;
            }
        }
        -1
    }
}
