use std::collections::HashMap;

impl Solution {
    pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();
        for &n in &nums {
            *freq.entry(n).or_insert(0) += 1;
        }

        let mut freq_count = HashMap::new();
        for v in freq.values() {
            *freq_count.entry(*v).or_insert(0) += 1;
        }

        for n in &nums {
            if freq_count[&freq[n]] == 1 {
                return *n;
            }
        }

        -1
    }
}
