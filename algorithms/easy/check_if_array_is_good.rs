use std::collections::HashMap;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut ma = -1;
        for n in nums {
            ma = ma.max(n);
            *mp.entry(n).or_default() += 1;
        }
        
        mp[&ma] == 2 && (1..ma).all(|i| mp.get(&i) == Some(&1))
    }
}
