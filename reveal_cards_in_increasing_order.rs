// https://leetcode.com/problems/reveal-cards-in-increasing-order/

use std::collections::LinkedList;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck.clone();
        deck.sort();

        let mut q = LinkedList::new();

        for &n in deck.iter().rev() {
            if q.len() < 2 {
                q.push_front(n);
            } else {
                let top = q.pop_back().unwrap();
                q.push_front(top);
                q.push_front(n);
            }
        }

        q.iter().map(|x| *x).collect::<Vec<i32>>()
    }
}
