impl Solution {
    pub fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i64 {
        let mut events = Vec::new();
        for each in intervals {
            events.push((each[0], 1));
            events.push((each[1], -1));
        }

        events.sort_unstable_by_key(|e| (e.0, -e.1));

        let mut ans = 0;
        let mut curr = 0;
        for evt in events {
            curr += evt.1 as i64;
            ans += curr / 2;
        }
        ans
    }
}
