// https://leetcode.com/problems/number-of-recent-calls/

use std::collections::LinkedList;

struct RecentCounter {
    q: LinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            q: LinkedList::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.q.len() > 0 && self.q.front().unwrap() < &(t - 3000) {
            self.q.pop_front();
        }
        self.q.push_back(t);

        self.q.len() as i32
    }
}
