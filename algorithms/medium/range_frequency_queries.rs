use std::collections::HashMap;

struct RangeFreqQuery {
    size: usize,
    tree: Vec<HashMap<i32, i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        let tree = vec![HashMap::new(); arr.len() * 4];
        let mut r = Self { size: arr.len(), tree };
        r.build_tree(0, &arr, 0, arr.len() - 1);
        r
    }

    fn build_tree(&mut self, node: usize, arr: &Vec<i32>, start: usize, end: usize) {
        if (start == end) {
            self.tree[node] = HashMap::new();
            self.tree[node].insert(arr[start], 1);
            return;
        }

        let mid = (start + end) / 2;
        let left = node * 2 + 1;
        let right = node * 2 + 2;
        self.build_tree(left, arr, start, mid);
        self.build_tree(right, arr, mid + 1, end);
        let mut mp = self.tree[left].clone();
        for (k, v) in self.tree[right].iter() {
            *mp.entry(*k).or_insert(0) += *v; 
        }
        self.tree[node] = mp;
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        self.query_r(0, 0, self.size - 1, left as usize, right as usize, value)
    }

    fn query_r(&self, node: usize, start: usize, end: usize, l: usize, r: usize, value: i32) -> i32 {
        if (r < start || l > end) {
            return 0;
        }
        if (l <= start && end <= r) {
            return *self.tree[node].get(&value).unwrap_or(&0);
        }
        let mid = (start + end) / 2;
        let left = node * 2 + 1;
        let right = node * 2 + 2;
        self.query_r(left, start, mid, l, r, value) + self.query_r(right, mid + 1, end, l, r, value)
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
