use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sum(ct: &HashMap<i32, i32>, x: usize) -> i32 {
    let mut hp = BinaryHeap::new();
    for (k, v) in ct.iter() {
        hp.push(Reverse((v, k)));
        if hp.len() > x {
            hp.pop();
        }
    }
    
    let mut res = 0;
    for Reverse((v, k)) in hp.into_iter() {
        res += v * k;
    }
    res
}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let mut ct = HashMap::new();
        for i in 0..k {
            *ct.entry(nums[i]).or_insert(0) += 1;
        }
        let mut arr = Vec::new();
        arr.push(sum(&ct, x));

        for i in k..nums.len() {
            *ct.entry(nums[i]).or_insert(0) += 1;
            *ct.entry(nums[i - k]).or_insert(0) -= 1;
            arr.push(sum(&ct, x));
        }
        arr
    }
}
