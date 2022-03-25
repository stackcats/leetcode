impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by(|a, b| (a[0]-a[1]).cmp(&(b[0]-b[1])));
        let n = costs.len() / 2;
        let mut ans = 0;
        for i in 0..n {
            ans += costs[i][0];
            ans += costs[i+n][1];
        }
        ans
    }
}
