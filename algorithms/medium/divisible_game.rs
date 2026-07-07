use std::collections::HashSet;

fn kadane(nums: &Vec<i32>, k: i32) -> i64 {
    let mut cur = 0i64;
    let mut ans = i64::MIN;
    for i in 0..nums.len() {
        let v = if nums[i] >= k && nums[i] % k == 0 {
            nums[i]
        } else {
            -nums[i]
        } as i64;
        cur = v.max(cur + v);
        ans = ans.max(cur);
    }
    ans
}

impl Solution {
    pub fn divisible_game(nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();
        st.insert(2);

        for &(mut n) in &nums {
            let mut k = 1;
            while k * k <= n {
                if n % k == 0 {
                    st.insert(k);
                    st.insert(n / k);
                }
                k += 1;
            }
        }

        st.remove(&1);

        let mut max_diff = i64::MIN;
        let mut k = 2;
        for n in st {
            let diff = kadane(&nums, n);
            if max_diff < diff {
                max_diff = diff;
                k = n;
            } else if max_diff == diff {
                k = k.min(n);
            }
        }

        (max_diff * k as i64).rem_euclid(1000000007) as _
    }
}
