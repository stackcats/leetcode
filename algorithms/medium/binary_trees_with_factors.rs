use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut map: HashMap<i32, i64> = HashMap::new();
        let mut ans = 0;
        for i in 0..arr.len() {
            map.insert(arr[i], 1);
            for j in 0..i {
                if arr[i] % arr[j] != 0 {
                    continue;
                }
                if let Some(ct) = map.get(&(arr[i] / arr[j])) {
                    let x = ct * map[&arr[j]];
                    map.entry(arr[i]).and_modify(|v| *v += x);
                }
            }
            ans = (ans + map[&arr[i]]) % 1000000007;
        }
        ans as i32
    }
}
