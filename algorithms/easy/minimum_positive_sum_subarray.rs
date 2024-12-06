impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut prefix_sum = vec![0];
        for n in nums {
            prefix_sum.push(n + prefix_sum.last().unwrap());
        }

        let mut ans = i32::MAX;

        for i in l..=r {
            for w in prefix_sum.windows(i as usize + 1) {
                let sum = w.last().unwrap() - w.first().unwrap();
                if sum > 0 {
                    ans = ans.min(sum);
                }
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
