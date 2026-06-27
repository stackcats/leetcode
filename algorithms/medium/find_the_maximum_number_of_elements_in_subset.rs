use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        nums.into_iter()
            .for_each(|n| *mp.entry(n).or_insert(0) += 1);
        let mut ans = 1;
        for (&k, v) in &mp {
            if k == 1 {
                if v % 2 == 1 {
                    ans = ans.max(*v);
                } else {
                    ans = ans.max(*v - 1);
                }
                continue;
            }
            let mut ct = 0;
            let mut n = k;
            while let Some(&v) = mp.get(&n)
                && v > 1
            {
                n *= n;
                ct += 2;
            }

            ct += if mp.contains_key(&n) { 1 } else { -1 };

            ans = ans.max(ct);
        }
        ans
    }
}
