use std::collections::BTreeMap;

#[derive(Default, Debug)]
struct MultiSet<T> {
    ct: usize,
    mp: BTreeMap<T, i32>,
}

impl<T: Ord + Copy + Default> MultiSet<T> {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, elem: T) {
        *self.mp.entry(elem).or_default() += 1;
        self.ct += 1;
    }

    fn remove(&mut self, elem: &T) {
        if let Some(v) = self.mp.get_mut(elem) {
            *v -= 1;
            self.ct -= 1;
            if *v == 0 {
                self.mp.remove(&elem);
            }
        }
    }

    fn first(&self) -> Option<&T> {
        self.mp.first_key_value().map(|(k, _)| k)
    }

    fn last(&self) -> Option<&T> {
        self.mp.last_key_value().map(|(k, _)| k)
    }

    fn len(&self) -> usize {
        self.ct
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn pop_first(&mut self) -> Option<T> {
        let k = self.mp.first_key_value()?.0.clone();
        let v = self.mp.get_mut(&k).unwrap();
        *v -= 1;
        if *v == 0 {
            self.mp.remove(&k);
        }
        self.ct -= 1;
        Some(k)
    }

    fn pop_last(&mut self) -> Option<T> {
        let k = self.mp.last_key_value()?.0.clone();
        let v = self.mp.get_mut(&k).unwrap();
        *v -= 1;
        if *v == 0 {
            self.mp.remove(&k);
        }
        self.ct -= 1;
        Some(k)
    }

    fn contains(&self, elem: &T) -> bool {
        self.mp.contains_key(elem)
    }
}

#[derive(Default, Debug)]
struct OrderSet {
    left: MultiSet<i32>,
    right: MultiSet<i32>,
}

impl OrderSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, elem: i32) {
        if self.right.is_empty() || *self.right.first().unwrap() > elem {
            self.left.insert(elem);
        } else {
            self.right.insert(elem);
        }

        self.rebalance();
    }

    fn remove(&mut self, elem: &i32) {
        if self.left.contains(elem) {
            self.left.remove(elem);
        } else {
            self.right.remove(elem);
        }
        self.rebalance();
    }

    fn rebalance(&mut self) {
        while self.left.len() < self.right.len() {
            let n = self.right.pop_first().unwrap();
            self.left.insert(n);
        }

        while self.left.len() > self.right.len() + 1 {
            let n = self.left.pop_last().unwrap();
            self.right.insert(n);
        }
    }

    fn median(&self) -> f64 {
        let a = *self.left.last().unwrap() as f64;
        if self.left.len() != self.right.len() {
            a
        } else {
            let b = *self.right.first().unwrap() as f64;
            (a + b) / 2.0
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut st = OrderSet::new();
        for i in 0..k {
            st.insert(nums[i]);
        }

        let mut ans = Vec::new();
        ans.push(st.median());

        for i in k..nums.len() {
            st.remove(&nums[i - k]);
            st.insert(nums[i]);
            ans.push(st.median());
        }

        ans
    }
}
