use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<i64> {
        let mut arr = Vec::new();
        for (i, n) in nums1.into_iter().enumerate() {
            arr.push((n, nums2[i] as i64, i));
        }

        arr.sort();

        let mut hp = BinaryHeap::new();
        let mut prev = 0;
        let mut prev_sum = 0;
        let mut sum = 0;
        let mut ans = vec![0; arr.len()];
        for (n, n2, i) in arr {
            if n == prev {
                ans[i] = prev_sum;
            } else {
                ans[i] = sum;
                prev_sum = sum;
            }

            hp.push(Reverse(n2));
            sum += n2;
            if (hp.len() > k as usize) {
                sum -= hp.peek().unwrap().0;
                hp.pop();
            }
            prev = n;
        }

        ans
    }
}
