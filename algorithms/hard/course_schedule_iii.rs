use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut total = 0;
        courses.sort_by_key(|k| k[1]);
        for course in courses {
            total += course[0];
            pq.push(course[0]);
            if total > course[1] {
                total -= pq.pop().unwrap();
            }
        }
        pq.len() as i32
    }
}
