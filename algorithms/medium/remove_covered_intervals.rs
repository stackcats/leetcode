impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| (v[0], -v[1]));

        intervals
            .iter()
            .fold((0, 0), |(acc, prev), v| {
                if v[1] > prev {
                    (acc + 1, v[1])
                } else {
                    (acc, prev)
                }
            })
            .0
    }
}
