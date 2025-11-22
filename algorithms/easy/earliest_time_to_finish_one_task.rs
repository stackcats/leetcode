impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks
            .into_iter()
            .min_by_key(|v| v[0] + v[1])
            .map(|v| v[0] + v[1])
            .unwrap()
    }
}
