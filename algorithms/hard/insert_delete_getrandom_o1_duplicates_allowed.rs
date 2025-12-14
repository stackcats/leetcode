use rand::{rngs::ThreadRng, Rng};
use std::collections::{BinaryHeap, HashMap};

struct RandomizedCollection {
    elems: Vec<i32>,
    mp: HashMap<i32, BinaryHeap<usize>>,
    r: ThreadRng,
}

impl RandomizedCollection {
    fn new() -> Self {
        Self {
            elems: Vec::new(),
            mp: HashMap::new(),
            r: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.elems.push(val);
        let ndx = self.elems.len() - 1;
        let ret = self.mp.contains_key(&val);
        self.mp.entry(val).or_insert(BinaryHeap::new()).push(ndx);
        !ret
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.mp.contains_key(&val) {
            return false;
        }

        let v = self.mp.get_mut(&val).unwrap();
        let ndx = v.pop().unwrap();
        if v.len() == 0 {
            self.mp.remove(&val);
        }

        let last_val = self.elems.last().unwrap();
        if val != *last_val {
            let last_ndx = self.mp.get_mut(last_val).unwrap();
            last_ndx.pop();
            last_ndx.push(ndx);
        }
        self.elems.swap_remove(ndx);
        true
    }

    fn get_random(&mut self) -> i32 {
        let idx = self.r.gen_range(0..self.elems.len());
        *self.elems.get(idx).unwrap()
    }
}
