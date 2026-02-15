impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut v = vec![false; 102];
        for bulb in bulbs {
            v[bulb as usize] = !v[bulb as usize];
        }
        let mut ans = Vec::new();
        for (i, &b) in v.iter().enumerate() {
            if b {
                ans.push(i as i32);
            }
        }

        ans
    }
}
