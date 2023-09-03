use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut mp = HashMap::new();
        let mut st = HashSet::new();

        for n in &nums {
            *mp.entry(n).or_insert(0) += 1;
        }

        let mut ans = Vec::new();

        for n in &nums {
            let v = mp.get_mut(n).unwrap();
            *v -= 1;
            if *v == 0 {
                mp.remove(n);
            }

            st.insert(n);

            ans.push((st.len() - mp.len()) as i32);
        }

        ans
    }
}
