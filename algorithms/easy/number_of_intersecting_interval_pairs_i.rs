impl Solution {
    pub fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..intervals.len() {
            for j in i + 1..intervals.len() {
                let (a, b) = (intervals[i][0], intervals[i][1]);
                let (c, d) = (intervals[j][0], intervals[j][1]);
                if c <= a && a <= d || a <= c && c <= b {
                    ans += 1;
                }
            }
        }
        ans
    }
}
