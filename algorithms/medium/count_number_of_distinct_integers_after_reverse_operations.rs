use std::collections::HashSet;

fn re(mut n: i32) -> i32 {
    let mut m = 0;
    while n > 0 {
        m = m * 10 + n % 10;
        n /= 10;
    }
    m
}

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<_> = nums.iter().cloned().collect();
        for n in nums {
            set.insert(re(n));
        }
        set.len() as _
    }
}
