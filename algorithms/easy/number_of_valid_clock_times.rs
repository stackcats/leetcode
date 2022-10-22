impl Solution {
    pub fn count_time(time: String) -> i32 {
        let time = time.as_bytes();
        let mut ct = match (time[0], time[1]) {
            (b'?', b'?') => 24,
            (b'?', b'0'..=b'3') => 3,
            (b'?', _) => 2,
            (b'2', b'?') => 4,
            (_, b'?') => 10,
            _ => 1,
        };
        if time[3] == b'?' {
            ct *= 6;
        }
        if time[4] == b'?' {
            ct *= 10;
        }
        ct
    }
}
