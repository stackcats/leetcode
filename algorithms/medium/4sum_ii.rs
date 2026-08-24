use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut mp = HashMap::new();
        for n1 in nums1 {
            for &n2 in &nums2 {
                *mp.entry(n1 + n2).or_insert(0) += 1;
            }
        }

        let mut ans = 0;
        for n3 in nums3 {
            for &n4 in &nums4 {
                if let Some(v) = mp.get(&(-n3 - n4)) {
                    ans += *v;
                }
            }
        }

        ans
    }
}
