impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut mi = i32::MAX;
        let mut ans = Vec::new();
        for c in cost {
            mi = mi.min(c);
            ans.push(mi);
        }
        ans
    }
}
