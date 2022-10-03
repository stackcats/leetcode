impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let mut ans = 0;
        let mut sum = needed_time[0];
        let mut max = needed_time[0];
        for i in 1..colors.len() {
            if colors[i] != colors[i - 1] {
                ans += (sum - max);
                sum = 0;
                max = 0;
            }
            sum += needed_time[i];
            max = max.max(needed_time[i]);
        }
        ans + sum - max
    }
}
