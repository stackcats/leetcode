use std::collections::HashMap;

fn reverse(mut n: i32) -> i32 {
    let mut m = 0;
    while n > 0 {
        m = m * 10 + n % 10;
        n /= 10;
    }
    m
}

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();

        let mut ans = usize::MAX;

        for (j, n) in nums.into_iter().enumerate() {
            if let Some(&i) = mp.get(&n) {
                ans = ans.min(j - i);
            }
            mp.insert(reverse(n), j);
        }

        if ans == usize::MAX { -1 } else { ans as _ }
    }
}
