use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut empty_rooms: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();

        let mut allocated_rooms = BinaryHeap::new();

        meetings.sort();

        let mut ct = vec![0; n];

        for meeting in meetings {
            while let Some(&Reverse((end, room))) = allocated_rooms.peek() {
                if end > meeting[0] as i64 {
                    break;
                }

                allocated_rooms.pop();
                empty_rooms.push(Reverse(room));
            }

            if let Some(Reverse(room)) = empty_rooms.pop() {
                ct[room] += 1;
                allocated_rooms.push(Reverse((meeting[1] as i64, room)));
            } else if let Some(Reverse((end, room))) = allocated_rooms.pop() {
                ct[room] += 1;
                let duration = (meeting[1] - meeting[0]) as i64;
                allocated_rooms.push(Reverse((end + duration, room)));
            }
        }

        (0..n).max_by_key(|&i| (ct[i], Reverse(i))).unwrap() as _
    }
}
