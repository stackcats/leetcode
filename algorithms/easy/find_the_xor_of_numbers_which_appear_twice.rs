use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let st: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        st.into_iter()
            .chain(nums.into_iter())
            .reduce(|acc, e| acc ^ e)
            .unwrap()
    }
}
