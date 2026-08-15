use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ct1 = HashMap::new();
        for n in nums1 {
            *ct1.entry(n).or_insert(0) += 1;
        }

        let mut ct2 = HashMap::new();
        for n in nums2 {
            if let Some(v) = ct1.get_mut(&n) {
                *v -= 1;
                if *v == 0 {
                    ct1.remove(&n);
                }
            } else {
                *ct2.entry(n).or_insert(0) += 1;
            }
        }

        let mut ans = 0;
        for v in ct1.values() {
            if v % 2 == 1 {
                return -1;
            }
            ans += v / 2;
        }

        for v in ct2.values() {
            if v % 2 == 1 {
                return -1;
            }
        }

        ans
    }
}
