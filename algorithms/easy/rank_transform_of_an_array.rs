use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            return vec![];
        }
        let mut sort_arr = arr.clone();
        sort_arr.sort();
        let mut rank = HashMap::new();
        let mut last_rank = 1;
        rank.insert(sort_arr[0], last_rank);
        for i in 1..sort_arr.len() {
            if sort_arr[i] != sort_arr[i - 1] {
                last_rank += 1;
                rank.insert(sort_arr[i], last_rank);
            }
        }
        let mut ans = Vec::new();
        for a in &arr {
            ans.push(rank[a]);
        }
        ans
    }
}
