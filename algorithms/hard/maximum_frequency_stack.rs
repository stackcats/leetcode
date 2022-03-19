use std::collections::{BinaryHeap, HashMap};

struct FreqStack {
    size: i32,
    q: BinaryHeap<(i32, i32, i32)>,
    m: HashMap<i32, i32>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            q: BinaryHeap::new(),
            m: HashMap::new(),
            size: 0,
        }
    }

    fn push(&mut self, val: i32) {
        self.size += 1;
        let ct = self.m.entry(val).or_insert(0);
        *ct += 1;
        self.q.push((*ct, self.size, val));
    }

    fn pop(&mut self) -> i32 {
        let (_, _, val) = self.q.pop().unwrap();
        self.m.entry(val).and_modify(|ct| *ct -= 1);
        val
    }
}
