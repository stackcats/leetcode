impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut pre_min = arrays[0][0];
        let mut pre_max = arrays[0].last().copied().unwrap();
        let mut ans = 0;
        for i in 1..arrays.len() {
            let min = arrays[i][0];
            let max = arrays[i].last().copied().unwrap();
            ans = ans.max(pre_max - min).max(max - pre_min);
            pre_min = pre_min.min(min);
            pre_max = pre_max.max(max);
        }
        ans
    }
}
