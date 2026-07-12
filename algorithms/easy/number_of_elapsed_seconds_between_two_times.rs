fn h(time: String) -> i32 {
    let time: Vec<_> = time.split(":").map(|s| s.parse::<i32>().unwrap()).collect();
    time[0] * 3600 + time[1] * 60 + time[2]
}

impl Solution {
    pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        h(end_time) - h(start_time)
    }
}
