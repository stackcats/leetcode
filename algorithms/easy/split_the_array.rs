impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut mp = vec![0; 101];
        for n in nums {
            let n = n as usize;
            if mp[n] >= 2 {
                return false;
            }
            mp[n] += 1
        }
        true
    }
}
