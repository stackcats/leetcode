use std::collections::HashSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in &nums {
            set.insert(*n);
        }
        let nums: Vec<i32> = set.into_iter().collect();
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }
        let mut a = nums[0];
        let mut b = nums[1];
        let mut c = nums[2];
        if a < b {
            let t = a;
            a = b;
            b = t;
        }
        if a < c {
            let t = a;
            a = c;
            c = t;
        }
        if b < c {
            let t = b;
            b = c;
            c = t;
        }
        for i in 3..nums.len() {
            if a < nums[i] {
                c = b;
                b = a;
                a = nums[i];
            } else if b < nums[i] && c != nums[i] {
                c = b;
                b = nums[i];
            } else if c < nums[i] && b != nums[i] && a != nums[i] {
                c = nums[i];
            }
        }
        c
    }
}
