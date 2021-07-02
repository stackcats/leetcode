use std::collections::HashSet;
impl Solution {
    pub fn longest_commom_subsequence(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set: HashSet<_> = arrays[0].iter().collect();
        for i in 1..arrays.len() {
            let set2: HashSet<_> = arrays[i].iter().collect();
            set = set.intersection(&set2).map(|n| *n).collect();
        }
        let mut ans: Vec<i32> = set.into_iter().map(|n| *n).collect();
        ans.sort();
        ans
    }
}
