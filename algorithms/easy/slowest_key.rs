impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys_pressed_arr = keys_pressed.chars().collect::<Vec<char>>();
        let mut c = keys_pressed_arr[0];
        let mut largest = release_times[0];
        for i in 1..release_times.len() {
            let release_time = release_times[i] - release_times[i - 1];
            if largest < release_time {
                largest = release_time;
                c = keys_pressed_arr[i];
            } else if largest == release_time {
                if c < keys_pressed_arr[i] {
                    c = keys_pressed_arr[i];
                }
            }
        }
        c
    }
}
