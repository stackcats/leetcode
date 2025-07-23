use std::collections::HashMap;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        for &n in &nums {
            mp.insert(n, 1);
        }

        let k = mp.len();
        let mut ans = 0;

        let mut i = 0;
        let mut mp = HashMap::new();
        for j in 0..nums.len() {
            *mp.entry(nums[j]).or_insert(0) += 1;
            while mp.len() == k {
                ans += nums.len() - j;
                let p = mp.get_mut(&nums[i]).unwrap();
                *p -= 1;
                if *p == 0 {
                    mp.remove(&nums[i]);
                }
                i += 1;
            }
        }

        ans as i32
    }
}
