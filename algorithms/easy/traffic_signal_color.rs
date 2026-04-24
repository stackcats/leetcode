impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        let s = if timer == 0 {
            "Green"
        } else if timer == 30 {
            "Orange"
        } else if timer > 30 && timer <= 90 {
            "Red"
        } else {
            "Invalid"
        };

        s.to_string()
    }
}
