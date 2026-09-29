use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, sum: i32) -> i32 {
        let mut dp = vec![i32::MAX; sum as usize + 1];
        dp[0] = 0;

        for i in 0..nums.len() {
            let mut mp = HashMap::new();

            let mut v = nums[i];
            let mut ct = 0;

            while v <= sum {
                mp.insert(v, ct);
                v *= 2;
                ct += 1;
            }

            v = nums[i];
            ct = 0;
            while v >= 1 {
                v /= 2;
                ct += 1;
                if v <= sum {
                    mp.insert(v, ct);
                }
            }

            let mut next = dp.clone();
            for (v, ct) in mp {
                for j in 0..=sum {
                    if dp[j as usize] == i32::MAX || j + v > sum {
                        continue;
                    }

                    next[(j + v) as usize] = next[(j + v) as usize].min(dp[j as usize] + ct);
                }
            }
            dp = next
        }

        if dp[sum as usize] == i32::MAX {
            -1
        } else {
            dp[sum as usize]
        }
    }
}
