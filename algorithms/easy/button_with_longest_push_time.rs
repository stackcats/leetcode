use std::cmp::Ordering;

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut i = events[0][0];
        let mut time = events[0][1];
        for event in events.into_iter().drop() {
            let t = events[j][1] - events[j - 1][1];
            match t.cmp(&time) {
                Ordering::Greater => {
                    i = events[j][0];
                    time = t;
                }
                Ordering::Equal => {
                    i = i.min(events[j][0]);
                }
                _ => (),
            }
        }
        i
    }
}
