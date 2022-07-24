struct FenwickTree {
    v: Vec<i32>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self { n: n, v: vec![0; n] }
    }

    fn add(&mut self, mut i: usize, val: i32) {
        while i < self.n {
            self.v[i] += val;
            i += 1 << i.trailing_zeros();
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.v[i];
            i -= 1 << i.trailing_zeros();
        }
        sum
    }
}

struct NumArray {
    nums: Vec<i32>,
    tree: FenwickTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = FenwickTree::new(nums.len() + 1);
        for i in 0..nums.len() {
            tree.add(i + 1, nums[i]);
        }
        Self { nums, tree }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let diff = val - self.nums[index];
        self.nums[index] = val;
        self.tree.add(index + 1, diff);
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        self.tree.query(right + 1) - self.tree.query(left)
    }
}
